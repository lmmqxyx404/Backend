use crate::error::Result;
use futures_util::future::BoxFuture;
use std::time::Duration;

use super::ApplicationConfig;
use crate::service::{MemService, RedisService};
pub trait ICacheService: Send + Sync {
    fn get_string(&self, k: &str) -> BoxFuture<Result<String>>;
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>>;
    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>>;
    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg: &ApplicationConfig) -> Result<Self> {
        match cfg.cache_type.as_str() {
            "mem" => {
                println!("[backend] cache type: mem");
                panic!("hello");
                Ok(Self {
                    inner: Box::new(MemService::default()),
                })
            }
            "redis" => Ok(Self {
                inner: Box::new(RedisService::new(&cfg.redis_url)),
            }),
            e => {
                panic!("unknown cache_type:{}", e);
            }
        }
    }
}
