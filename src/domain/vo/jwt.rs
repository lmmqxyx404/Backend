use crate::error::Error;
use serde::{Deserialize, Serialize};

/// JWT 鉴权 Token结构(Json Web Tokens)
pub struct JWT_Token {
    /// 账号id
    pub id: String,
    /// 账号
    pub account: String,
    /// 权限集合
    pub permissions: String,
    /// 角色id集合
    pub role_ids: String,
    /// 过期时间
    pub exp: usize,
}

impl JWT_Token {

}