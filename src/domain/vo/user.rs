use serde::{Deserialize, Serialize};

use crate::{
    domain::table::{LoginCheck, SysUser},
    service::CONTEXT,
};
/// 4.0 之后的 rabatis 才支持
use rbatis::rbdc::datetime::DateTime;

use super::role::SysRoleVO;
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
    // pub del: Option<i32>,
    pub create_date: Option<String>,
    /// 注意类型
    pub role: Option<SysRoleVO>,
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
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            role: None,
        }
    }
}
