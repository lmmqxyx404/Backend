use serde::{Deserialize, Serialize};

/// 4.0 之后的 rabatis 才支持
use rbatis::rbdc::datetime::DateTime;
// use crate::domain::dto::sign_in::SignDTO;
/// 后台用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<String>,
    pub state: Option<String>,
    pub del: Option<String>,
    pub create_date: Option<DateTime>,
    pub role: Option<String>,
}
