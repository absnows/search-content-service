use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;

use routers::content_birth_routers::ContentBirthRouter;
use routers::ping_controller::PingController;

pub mod routers;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let bind_address = env::var("BIND_ADDRESS").expect("Any bind_address was configured");

    let _ = HttpServer::new(|| App::new().wrap(Logger::default()).configure(routes))
        .bind(&bind_address)
        .expect("Erro to up server")
        .run()
        .await;

    log::info!("Server run on address: {}", bind_address);
    Ok(())
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