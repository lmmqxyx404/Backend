// use actix_web::guard;
use parking_lot::Mutex;
use std::{
    collections::{hash_map::RandomState, HashMap},
    ops::Sub,
    time::{Duration, Instant},
};

use crate::error::{Error, Result};
use crate::service::cache_service::ICacheService;
use futures_util::future::BoxFuture;

/// 内存缓存服务
/// 这个服务还有很多细节不会
pub struct MemService {
    pub cache: Mutex<HashMap<String, (String, Option<(Instant, Duration)>), RandomState>>,
}

impl MemService {
    pub fn recycling(&self) {
        let mut map_lock_guard = self.cache.lock();
        let mut need_removed = vec![];

        for (k, v) in map_lock_guard.iter() {
            if let Some((i, d)) = v.1 {
                if i.elapsed() >= d {
                    need_removed.push(k.to_string());
                }
            }
        }
        for x in need_removed {
            map_lock_guard.remove(&x);
        }
    }
}

impl Default for MemService {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl ICacheService for MemService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let (k, v) = (k.to_string(), v.to_string());
        let mut guard = self.cache.lock();
        guard.insert(k.to_string(), (v.clone(), None));
        Box::pin(async move {
            return Ok(v.to_string());
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        self.recycling();
        let k = k.to_string();
        let guard = self.cache.lock();
        let mut v = String::new();
        if let Some(r) = guard.get(&k) {
            v = r.0.to_string();
        }
        Box::pin(async move { Ok(v) })
    }

    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>> {
        self.recycling();
        let (k, v, mut e, mut locked) = (k.to_string(), v.to_string(), None, self.cache.lock());
        if let Some(ex) = ex {
            e = Some((Instant::now(), ex));
        }
        let inserted = locked.insert(k, (v.clone(), e));
        Box::pin(async move {
            if inserted.is_some() {
                return Ok(v.to_string());
            }
            return Err(Error::E(format!("[backend][mem_service] insert failed!")));
        })
    }

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        self.recycling();
        let locked = self.cache.lock();
        let v = locked.get(k).cloned();
        drop(locked);
        let v = match v {
            None => -2,
            Some((r, o)) => match o {
                None => -1,
                Some((i, d)) => {
                    let use_time = i.elapsed();
                    if d > use_time {
                        d.sub(use_time).as_secs() as i64
                    } else {
                        0
                    }
                }
            },
        };
        Box::pin(async move { Ok(v) })
    }
}
