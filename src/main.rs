use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolOptions, sqlite::SqliteConnectOptions, sqlite::SqlitePool};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Wordl!")
}

#[get("/user/{username}")]
async fn login(path: web::Path<usize>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id: i64 = path.into_inner() as i64;
    #[derive(serde::Serialize, Deserialize)]
    struct User {
        id: i64,
        username: String,
        password: String,
    }

    let user: Option<User> = sqlx::query_as!(User, "SELECT * FROM User WHERE id = ?", user_id)
        .fetch_optional(&app_state.pool)
        .await
        .unwrap();

    match user {
        Some(x) => x.username.to_string(),
        None => "not ok".to_string(),
    }
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
    })
    .bind((address, port))?
    .run()
    .await
}
