mod api;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use crate::api::search::search_by_date;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger configuration
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Setup Server
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().wrap(logger)
        .service(search_by_date)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
