use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use crate::routers::content_birth_routers::ContentBirthRouter;

pub mod routers;

#[get("/")]
async fn get() -> impl Responder {
    ContentBirthRouter::get_router().await;
    HttpResponse::Ok().body("Hello from this get")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Call function from get resource external
    ContentBirthRouter::get_router().await;

    HttpServer::new(|| App::new().service(get))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
