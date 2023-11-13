use actix_web::{body::None, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolOptions, sqlite::SqlitePool};

use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(serde::Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    password: String,
    cover: Option<String>,
}

#[derive(serde::Serialize, Deserialize)]
struct Song {
    id: i64,
    name: String,
    release: String,
    album: String,
    cover: Option<String>,
    location: String,
}

#[derive(serde::Serialize, Deserialize)]
struct Artist {
    id: i64,
    name: String,
    cover: Option<String>,
    about: String,
}

#[get("/get/file/{filename:.*}")]
async fn hostfile(req: web::Path<String>) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = format!("/home/zain/dbms_project/assets/{}", req.to_string())
        .parse()
        .unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome To Munna's Music")
}

#[get("/get/song")]
async fn get_songs(app_state: web::Data<AppState>) -> HttpResponse {
    let songs: Vec<Song> = sqlx::query_as!(Song, "SELECT  * FROM Songs")
        .fetch_all(&app_state.pool)
        .await
        .unwrap();
    HttpResponse::Ok().json(songs)
}

#[post("/post/song")]
async fn post_songs(body: web::Json<Song>, app_state: web::Data<AppState>) -> HttpResponse {
    let insert_song = sqlx::query!(
        "INSERT INTO Songs VALUES(?, ?,?,?,?,?)",
        body.id,
        body.name,
        body.release,
        body.album,
        body.cover,
        body.location
    )
    .execute(&app_state.pool)
    .await;
    if let Err(_error) = insert_song {
        HttpResponse::NotAcceptable().into()
    } else {
        HttpResponse::Ok().into()
    }
}

#[get("/search/{title}")]
async fn search_song(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let sub_str = path.to_string();
    let sub_search = format!("%{}%", sub_str);
    let songs: Vec<Song> =
        sqlx::query_as!(Song, "SELECT * FROM Songs WHERE name LIKE ?", sub_search)
            .fetch_all(&app_state.pool)
            .await
            .unwrap();
    if songs.len() == 0 {
        HttpResponse::NotFound().into()
    } else {
        HttpResponse::Ok().json(songs)
    }
}

#[post("/login")]
async fn login(body: web::Json<User>, app_state: web::Data<AppState>) -> HttpResponse {
    let is_user: Option<User> = sqlx::query_as!(
        User,
        "SELECT * FROM User WHERE username = ? and password = ?",
        body.username,
        body.password
    )
    .fetch_optional(&app_state.pool)
    .await
    .unwrap();

    match is_user {
        Some(expr) => HttpResponse::Ok().json(expr),
        None => HttpResponse::NotFound().into(),
    }
}

#[get("/user/{username}")]
async fn username(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse {
    // let user_id: i64 = path.into_inner() as i64;
    let user_name = path.to_string();

    //To fetch all
    // let user: Vec<User> = sqlx::query_as!(User, "SELECT * FROM User WHERE username = ?", user_name)
    //     .fetch_all(&app_state.pool)
    //     .await
    //     .unwrap();

    let user: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM User WHERE username = ?", user_name)
            .fetch_optional(&app_state.pool)
            .await
            .unwrap();

    match user {
        Some(x) => HttpResponse::Ok().json(x),
        None => HttpResponse::BadRequest().into(),
    }
    // HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;

    let pool: SqlitePool = PoolOptions::new().connect("test.db").await.unwrap();

    let app_state = AppState { pool };

    println!("Server Starting at http://{}:{}", address, port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(hello)
            .service(login)
            .service(get_songs)
            .service(post_songs)
            .service(search_song)
            .service(hostfile)
    })
    .bind((address, port))?
    .run()
    .await
}
