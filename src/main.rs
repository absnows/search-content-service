use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger configuration
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().wrap(logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
