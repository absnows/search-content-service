use actix_web::{web, HttpResponse, Responder};
use redis::Commands;

#[derive(Debug)]
pub struct ContentBirthRouter {}

impl ContentBirthRouter {
    pub async fn get(path: web::Path<String>) -> impl Responder {
        let birthdate = path.into_inner();
        let redis_url = "redis://127.0.0.1/";
        let client = redis::Client::open(redis_url).expect("File to open client configuration");
        let mut connection = client
            .get_connection()
            .expect("Failed to generate connection ");

        let content: String = connection
            .get(birthdate)
            .expect("Failed to search date value");

        HttpResponse::Ok().body(content)
    }
}
