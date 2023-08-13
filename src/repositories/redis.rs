use std::time::Duration;

use r2d2_redis::{r2d2::Pool, redis::Commands, RedisConnectionManager};

const CACHE_POOL_MAX_OPEN: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

#[derive(Debug)]
pub struct RedisConnection {
    connection_url: String,
}

impl RedisConnection {
    pub fn new(connection_url: String) -> Self {
        RedisConnection { connection_url }
    }

    pub fn connection(&self) -> Pool<RedisConnectionManager> {
        let manager = RedisConnectionManager::new(self.connection_url.clone())
            .expect("Error to generate RedisConnectionManager");
        r2d2::Pool::builder()
            .max_size(CACHE_POOL_MAX_OPEN)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .min_idle(Some(CACHE_POOL_MIN_IDLE))
            .build(manager)
            .expect("Error to make connection")
    }
}

#[derive(Debug)]
pub struct RedisRepository {
    pool: Pool<RedisConnectionManager>,
}

impl RedisRepository {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        RedisRepository { pool }
    }

    pub fn get(&self, key: String) -> Option<String> {
        log::info!("[flow:redis-get][message:search for {}]", key);
        let mut connection = self.pool.get().unwrap();
        let r = connection.get(key).unwrap();
        return r;
    }
}
