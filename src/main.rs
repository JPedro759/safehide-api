mod types;
mod models;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
