use std::sync::Arc;

use crate::repositories::repository::Repository;

use super::search_error::SearchError;
use super::usecases::search_use_case::SearchUseCase;
use super::usecases::validate_date_use_case::validate;

pub struct SearchDateService {
    repository: Arc<dyn Repository<String> + 'static>,
}

impl SearchDateService {
    pub fn new(repository: Arc<dyn Repository<String> + 'static>) -> Self {
        SearchDateService { repository }
    }

    pub fn search(&self, date_request: String) -> Result<String, SearchError> {
        let repo: Arc<dyn Repository<String>> = self.repository.clone();
        let usecase: SearchUseCase<String> = SearchUseCase::new(repo);

        if validate(&date_request) {
            return match usecase.search(date_request) {
                Ok(result) => Ok(result),
                Err(err) => Err(err),
            };
        }

        Err(SearchError::InvalidSearchDateFormatError)
    }
}
