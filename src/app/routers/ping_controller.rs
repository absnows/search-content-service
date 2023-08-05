use actix_web::{HttpResponse, Responder};

#[derive(Debug)]
pub struct PingController {}

impl PingController {
    pub async fn pong() -> impl Responder {
        HttpResponse::Ok().body("pong")
    }
}
