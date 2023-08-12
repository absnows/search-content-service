use std::time::Duration;

use r2d2::Pool;
use r2d2_redis::{redis::Commands, RedisConnectionManager};

const CACHE_POOL_MAX_OPEN: u32 = 16;
const CACHE_POOL_MIN_IDLE: u32 = 8;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub struct RedisConnection {
    connection_url: String,
}

impl RedisConnection {
    pub fn new(connection_url: String) -> RedisConnection {
        RedisConnection { connection_url }
    }

    pub fn connection(&self) -> Pool<RedisConnectionManager> {
        let manager =
            RedisConnectionManager::new(self.connection_url.clone()).expect("Error on connection");
        r2d2::Pool::builder()
            .max_size(CACHE_POOL_MAX_OPEN)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .min_idle(Some(CACHE_POOL_MIN_IDLE))
            .build(manager)
            .expect("Error on connection estabilish by configuration")
    }
}

pub struct RedisRepository {
    pool: Pool<RedisConnectionManager>,
}

impl RedisRepository {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        RedisRepository { pool }
    }

    pub fn search(&self, key: &str) -> Option<String> {
        match self.pool.get() {
            Ok(c) => {
                let mut connection = c;
                connection.get(key).unwrap()
            }
            Err(err) => {
                log::error!(
                    "[flow:search-get-by-key] failed into connection pool {:#?}",
                    err
                );
                None
            }
        }
    }
}
