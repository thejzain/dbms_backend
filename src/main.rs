use actix_web::{body::None, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolOptions, sqlite::SqlitePool};

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
}

#[derive(serde::Serialize, Deserialize)]
struct Artist {
    id: i64,
    name: String,
    cover: Option<String>,
    about: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Wordl!")
}

#[get("/get/songs")]
async fn get_songs(app_state: web::Data<AppState>) -> HttpResponse {
    let songs: Vec<Song> = sqlx::query_as!(Song, "SELECT  * FROM Songs")
        .fetch_all(&app_state.pool)
        .await
        .unwrap();
    HttpResponse::Ok().json(songs)
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
    })
    .bind((address, port))?
    .run()
    .await
}
