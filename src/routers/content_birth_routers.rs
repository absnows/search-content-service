use actix_web::{web, HttpResponse, Responder};

#[derive(Debug)]
pub struct ContentBirthRouter {}

impl ContentBirthRouter {
    pub async fn get(path: web::Path<String>) -> impl Responder {
        let birthdate = path.into_inner();
        dbg!(&birthdate);
        HttpResponse::Ok().body("Get from here...")
    }
}
