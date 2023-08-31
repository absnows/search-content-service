pub trait ExternalSearch {
    fn search_by_date(&self, date: &str) -> String;
}

pub struct ExternalSearchOpenAI {}

impl ExternalSearch for ExternalSearchOpenAI {
    fn search_by_date(&self, _date: &str) -> String {
        "mock-return".to_string()
    }
}
pub struct ExternalSearchUsCase {}

impl ExternalSearchUsCase {
    pub fn search_for_external_api(
        &self,
        date: &str,
        external_reference: Box<dyn ExternalSearch>,
    ) -> String {
        external_reference.search_by_date(date)
    }
}
