/// 校验模块
pub mod auth;
/// 登录模块
pub mod sign_in;

/// 用户模块
pub mod user;

use serde::{Deserialize, Serialize};

// 后续可能会移动IdDTO

/// IdDTO
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdDTO {
    pub id: Option<String>,
}
