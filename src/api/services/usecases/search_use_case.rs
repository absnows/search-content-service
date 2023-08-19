use std::sync::Arc;

use crate::{api::services::search_error::SearchError, repositories::repository::Repository};

pub struct SearchUseCase<T> {
    repository: Arc<dyn Repository<T> + 'static>,
}

impl<T> SearchUseCase<T> {
    pub fn new(repository: Arc<dyn Repository<T> + 'static>) -> Self {
        SearchUseCase { repository }
    }

    pub fn search(&self, request_date: T) -> Result<String, SearchError> {
        match self.repository.get(request_date) {
            Some(value) => Ok(value),
            None => Err(SearchError::NoContentWasFoundError),
        }
    }
}

// Define a FakeRepository for testing purposes
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

#[test]
fn test_search_use_case_with_value() {
    let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepository);
    let search_use_case = SearchUseCase::new(fake_repo);

    let result = search_use_case.search("request_key".to_string()).unwrap();

    assert_eq!(result, "Test Value".to_string());
}

#[test]
fn test_search_use_case_with_no_value() {
    let fake_repo: Arc<dyn Repository<String>> = Arc::new(FakeRepositoryFailt);
    let search_use_case = SearchUseCase::new(fake_repo);

    let result = search_use_case.search("nonexistent_key".to_string());

    assert!(result.is_err())
}
