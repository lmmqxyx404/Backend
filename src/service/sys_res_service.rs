use std::collections::BTreeMap;

use crate::{
    domain::{table::SysRes, vo::res::SysResVO},
    error::{Error, Result},
};

use super::CONTEXT;

const RES_KEY: &'static str = "sys_res:all";
/// 系统资源服务
pub struct SysResService {}

impl SysResService {
    pub async fn update_cache(&self) -> Result<Vec<SysResVO>> {
        
        Err(Error::from("temporary"))
    }
    pub async fn finds_all(&self) -> Result<Vec<SysResVO>> {
        let js = CONTEXT
            .cache_service
            .get_json::<Option<Vec<SysRes>>>(RES_KEY)
            .await;

        if js.is_err()
            || js.as_ref().unwrap().is_none()
            || js.as_ref().unwrap().as_ref().unwrap().is_empty()
        {
            let all = self.update_cache().await?;
            return Ok(all);
        }
        /// 之后可以添加代码实现打印日志
        let mut arr = vec![];
        if let Ok(v) = js {
            for x in v.unwrap_or(vec![]) {
                arr.push(x.into());
            }
        }
        Ok(arr)
        //Err(Error::from("temporary"))
    }

    pub async fn finds_all_map(&self) -> Result<BTreeMap<String, SysResVO>> {
        Err(Error::from("temporary"))
    }
}
