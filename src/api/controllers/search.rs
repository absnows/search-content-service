use actix_web::get;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use serde::Serialize;

use crate::api::services::search_date_service::SearchDateService;
use crate::api::services::search_error::SearchError;
use crate::repositories::redis::RedisRepository;

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    content: String,
}

impl SearchResponse {
    pub fn new(content: String) -> Self {
        SearchResponse { content }
    }
}

#[get("/api/dates/{value}")]
pub async fn search_by_date(
    db: Data<RedisRepository>,
    path: Path<String>,
) -> Result<Json<SearchResponse>, SearchError> {
    let date_to_search: String = path.into_inner();
    let service = SearchDateService::new(db.into_inner());

    match service.search(date_to_search) {
        Ok(result) => {
            let response = SearchResponse::new(result.trim().to_string());
            Ok(Json(response))
        }
        Err(err) => Err(err),
    }
}
