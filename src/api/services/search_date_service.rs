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

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository;
    struct FakeRepositoryFailt;

    impl<T> Repository<T> for FakeRepository {
        fn get(&self, _key: T) -> Option<String> {
            Some("Test Value".to_string())
        }
    }

    impl<T> Repository<T> for FakeRepositoryFailt {
        fn get(&self, _key: T) -> Option<String> {
            None
        }
    }

    const DATE_MOCK_TEST: &str = "1900-01-01";

    #[test]
    fn test_search_with_value() {
        let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepository);
        let service = SearchDateService::new(fake_repo);

        let result = service.search(DATE_MOCK_TEST.to_string()).unwrap();
        assert_eq!(result, "Test Value".to_string())
    }

    #[test]
    fn test_search_with_no_value() {
        let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepositoryFailt);
        let service = SearchDateService::new(fake_repo);

        let result = service.search(DATE_MOCK_TEST.to_string());

        assert!(result.is_err())
    }
}
