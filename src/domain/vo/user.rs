use serde::{Deserialize, Serialize};

use crate::domain::table::{LoginCheck, SysUser};
/// 4.0 之后的 rabatis 才支持
use rbatis::rbdc::datetime::FastDateTime;
// use crate::domain::dto::sign_in::SignDTO;
/// 后台用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<FastDateTime>,
    pub role: Option<String>,
}

impl From<SysUser> for SysUserVO {
    fn from(arg: SysUser) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            password: None,
            name: arg.name,
            login_check: arg.login_check,
            state: arg.state,
            del: arg.del,
            create_date: arg.create_date,
            role: None,
        }
    }
}
