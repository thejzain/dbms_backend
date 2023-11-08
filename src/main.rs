use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Wordl!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 8080;
    println!("Server Starting at http://{}:{}", address, port);
    HttpServer::new(|| App::new().service(hello))
        .bind((address, port))?
        .run()
        .await
}
