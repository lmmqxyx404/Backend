use crate::error::{Error, Result};
use redis::{aio::Connection, RedisResult};

use crate::service::cache_service::ICacheService;
use futures_util::future::BoxFuture;
pub struct RedisService {
    pub client: redis::Client,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).unwrap();
        Self { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let connection = self.client.get_async_connection().await;
        if connection.is_err() {
            let e = format!("RedisService connect failure:{}", connection.err().unwrap());
            return Err(crate::error::Error::from(e));
        }
        return Ok(connection.unwrap());
    }
}

impl ICacheService for RedisService {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>> {
        let (k, v) = (k.to_string(), v.to_string());
        Box::pin(async move {
            return self.set_string_ex(&k, &v, None).await;
        })
    }

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            let result: RedisResult<Option<String>> =
                redis::cmd("GET").arg(&[&k]).query_async(&mut con).await;
            return match result {
                Ok(v) => Ok(v.unwrap_or_default()),
                Err(e) => Err(Error::from(format!(
                    "Redis service get string({}) fail:{}",
                    k,
                    e.to_string()
                ))),
            };
        })
    }

    /// 设置字符过期
    fn set_string_ex(
        &self,
        k: &str,
        v: &str,
        ex: Option<std::time::Duration>,
    ) -> BoxFuture<Result<String>> {
        let (k, v) = (k.to_string(), v.to_string());
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            return if ex.is_none() {
                match redis::cmd("SET").arg(&[k, v]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "Redis serive set string ex failed:{}",
                        e.to_string()
                    ))),
                }
            } else {
                match redis::cmd("SET")
                    .arg(&[&k, &v, "EX", &ex.unwrap().as_secs().to_string()])
                    .query_async(&mut conn)
                    .await
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "Redis serive set string ex failed:{}",
                        e.to_string()
                    ))),
                }
            };
        })
    }

    /// set_string 自动过期
    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>> {
        let k = k.to_string();
        Box::pin(async move {
            let mut conn = self.get_conn().await?;
            Box::pin(async move {
                let mut conn = self.get_conn().await?;
                return match redis::cmd("TTL").arg(&[k]).query_async(&mut conn).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::from(format!(
                        "Redis serive ttl failed:{}",
                        e.to_string()
                    ))),
                };
            })
        })
    }
}
