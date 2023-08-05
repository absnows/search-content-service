use std::env;

use crate::routers::content_birth_routers::ContentBirthRouter;
use crate::routers::ping_controller::PingController;

use actix_web::{middleware::Logger, web, App, HttpServer};

mod routers;

//TODO: Build a route to get information with Birth Date
//TODO: Search information from Key Value datababse
//TODO: Build fix prompt by country and language
//TODO: Integrate with some LLVM

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // environment log configration
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    let address = env::var("BIND_ADDRESS").expect("Any address was configured");

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes))
        .bind(&address)?
        .run()
        .await
}

fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/api")
            .service(
                web::resource("/dates/{birthdate}").route(web::get().to(ContentBirthRouter::get)),
            )
            .service(web::resource("/ping").route(web::get().to(PingController::pong))),
    );
}
