mod api;
mod repositories;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use crate::api::controllers::search::search_by_date;
use crate::repositories::redis::{RedisConnection, RedisRepository};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup logger configuration
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Setup Server
    HttpServer::new(move || {
        let logger = Logger::default();
        let connection_redis = RedisConnection::new("redis://127.0.0.1/".to_string()); //TODO: Add environment configuration
        let repository = RedisRepository::new(connection_redis.connection());
        let redis_data = Data::new(repository);

        App::new()
            .wrap(logger)
            .app_data(redis_data)
            .service(search_by_date)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
