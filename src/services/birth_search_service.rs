#[derive(Debug)]
pub struct BirthSearchService {
    id: i32,
}

impl BirthSearchService {
    pub fn new() -> BirthSearchService {
        BirthSearchService { id: 1 }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}
