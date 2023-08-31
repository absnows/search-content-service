use std::sync::Arc;

use crate::repositories::repository::Repository;

use super::search_error::SearchError;
use super::usecases::external_search_use_case::{ExternalSearchOpenAI, ExternalSearchUsCase};
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
            return match usecase.search(date_request.clone()) {
                Ok(result) => Ok(result),
                Err(_) => self.external_search(date_request),
            };
        }

        Err(SearchError::InvalidSearchDateFormatError)
    }

    pub fn external_search(&self, date_request: String) -> Result<String, SearchError> {
        let external_open_ai_reference = Box::new(ExternalSearchOpenAI {});
        let external_search = ExternalSearchUsCase {};

        let content =
            external_search.search_for_external_api(&date_request, external_open_ai_reference);

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository;
    struct FakeRepositoryFail;

    impl<T> Repository<T> for FakeRepository {
        fn get(&self, _key: T) -> Option<String> {
            Some("Test Value".to_string())
        }
    }

    impl<T> Repository<T> for FakeRepositoryFail {
        fn get(&self, _key: T) -> Option<String> {
            None
        }
    }

    #[test]
    fn test_search_with_value() {
        let request_date_mock = String::from("1900-01-01");
        let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepository);
        let service = SearchDateService::new(fake_repo);

        let result = service.search(request_date_mock).unwrap();
        assert_eq!(result, "Test Value".to_string())
    }

    #[test]
    fn test_search_with_no_value() {
        let request_date_mock = String::from("19ZZZ-31-01");
        let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepositoryFail);
        let service = SearchDateService::new(fake_repo);

        let result = service.search(request_date_mock);

        assert!(result.is_err())
    }
}
