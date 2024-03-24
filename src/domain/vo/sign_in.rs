use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        dto::sign_in::SignDTO,
        table::{LoginCheck, SysUser},
    },
    service::CONTEXT,
};

use super::role::SysRoleVO;
/// 登录相关信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<String>,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub role: Option<SysRoleVO>,
}

impl From<SysUser> for SignVO {
    fn from(value: SysUser) -> Self {
        Self {
            id: value.id,
            account: value.account,
            password: value.password,
            name: value.name,
            login_check: value.login_check,
            state: value.state,
            create_date: value
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            permissions: vec![],
            access_token: "".to_string(),
            role: None,
        }
    }
}

/// 转化为字符串
impl ToString for SignVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
