use serde::{Deserialize, Serialize};

use crate::domain::dto::sign_in::SignDTO;

use super::role::SysRoleVO;
/// 登录相关信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignVO {
    pub user: Option<SignDTO>,
    pub permissions: Vec<String>,
    pub access_token: String,
    pub role: Option<SysRoleVO>,
}

/// 转化为字符串
impl ToString for SignVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
