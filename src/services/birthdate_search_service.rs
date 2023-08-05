#[derive(Debug)]
pub struct BirthdateSearchService {}

impl BirthdateSearchService {
    pub fn new() -> BirthdateSearchService {
        println!("Hello from service");
        BirthdateSearchService {}
    }

    pub fn search_information_by_date(&self, date: &str) -> String {
        String::from(date)
    }
}
