use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::table::SysDict;

/// 权限资源表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<FastDateTime>,
}

impl From<SysDict> for SysDictVO {
    fn from(value: SysDict) -> Self {
        Self {
            id: value.id,
            name: value.name,
            code: value.code,
            state: value.state,
            create_date: value.create_date,
        }
    }
}
