/// 校验模块
pub mod auth;
/// 登录模块
pub mod sign_in;

/// 用户模块
pub mod user;

/// 权限管理模块
pub mod permission;

/// 角色模块
pub mod role;

pub mod dict;

use serde::{Deserialize, Serialize};

// 后续可能会移动IdDTO

/// IdDTO
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdDTO {
    pub id: Option<String>,
}

/// common dto EmptyDTO
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmptyDTO {}
