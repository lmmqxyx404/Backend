use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::table::SysRes;

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysResVO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
    pub childs: Option<Vec<SysResVO>>,
}

impl From<SysRes> for SysResVO {
    fn from(arg: SysRes) -> Self {
        Self {
            id: arg.id,
            parent_id: arg.parent_id,
            name: arg.name,
            permission: arg.permission,
            path: arg.path,
            del: arg.del,
            create_date: arg.create_date,
            childs: None,
        }
    }
}
