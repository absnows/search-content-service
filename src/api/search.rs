use actix_web::ResponseError;
use actix_web::HttpResponse;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::http::StatusCode;
use actix_web::get;
use actix_web::http::header::ContentType;
use derive_more::Display;
use regex::Regex;
use serde::Serialize;

// Regular expression pattern for yyyy-mm-dd format
const REGEX_DATE_PATTERN_VALIDATE: &str = r"^(?P<year>\d{4})-(?P<month>0[1-9]|1[0-2])-(?P<day>0[1-9]|[12][0-9]|3[01])$";


#[derive(Debug, Serialize)]
pub struct SearchResponse {
    code: String,
    content: String
}

impl SearchResponse {
    pub fn new(code: String, content: String) -> Self {
        SearchResponse { code, content }    
    }
}

#[derive(Debug, Display)]
pub enum SearchError {
    InvalidSearchDateFormatError,
}

impl ResponseError for SearchError {

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {        
            SearchError::InvalidSearchDateFormatError => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/dates/{value}")]
pub async fn search_by_date(path: Path<String>) -> Result<Json<SearchResponse>, SearchError> {
    let date_to_search :String = path.into_inner();

    match validate_date(&date_to_search) {
        true => {
            let response = SearchResponse::new("200".to_string(), date_to_search);
            Ok(Json(response))
        },
        false => Err(SearchError::InvalidSearchDateFormatError)
    }
}

fn validate_date(date_to_validate: &String) -> bool {
        let re = Regex::new(REGEX_DATE_PATTERN_VALIDATE).unwrap();
    
        if let Some(captures) = re.captures(date_to_validate) {
            let year = captures.name("year").unwrap().as_str().parse::<i32>().unwrap();
            let month = captures.name("month").unwrap().as_str().parse::<i32>().unwrap();
            let day = captures.name("day").unwrap().as_str().parse::<i32>().unwrap();
            
            // add validation for february
            if month == 2 && day > 28 {
                return false;
            }

            if year >= 1900 && month >= 1 && month <= 12 && day >= 1 && day <= 31 {
                return true
            }
        }

        false
}