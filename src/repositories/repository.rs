pub trait Repository<K> {
    fn get(&self, key: K) -> Option<String>;
}
