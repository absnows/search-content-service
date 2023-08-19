use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum SearchError {
    InvalidSearchDateFormatError,
    NoContentWasFoundError,
}

impl ResponseError for SearchError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_status_and_message())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SearchError::NoContentWasFoundError => StatusCode::NOT_FOUND,
            SearchError::InvalidSearchDateFormatError => StatusCode::BAD_REQUEST,
        }
    }
}

impl SearchError {
    fn to_status_and_message(&self) -> String {
        match self {
            SearchError::NoContentWasFoundError => "No content found".to_string(),
            SearchError::InvalidSearchDateFormatError => "Invalid search date format".to_string(),
        }
    }
}
