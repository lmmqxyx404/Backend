use serde::{Deserialize, Serialize};

use crate::domain::dto::sign_in::SignDTO;
/// 登录相关信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignVO{
    pub user:Option<SignDTO>
}