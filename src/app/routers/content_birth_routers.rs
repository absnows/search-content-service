use actix_web::{web, HttpResponse, Responder};

#[derive(Debug)]
pub struct ContentBirthRouter {}

impl ContentBirthRouter {
    pub async fn get(path: web::Path<String>) -> impl Responder {
        let content: String = path.into_inner();

        // search information with date
        HttpResponse::Ok().body(content)
    }
}
