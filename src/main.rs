use actix_web::{web, App, HttpServer};

use crate::routers::content_birth_routers::ContentBirthRouter;

pub mod routers;

//TODO: Build a route to get information with Birth Date
//TODO: Search information from Key Value datababse
//TODO: Build fix prompt by country and language
//TODO: Integrate with some LLVM

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/dates/{birthdate}", web::get().to(ContentBirthRouter::get))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
