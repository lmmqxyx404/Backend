use crate::error::Result;
use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
/// JWT 鉴权 Token结构(Json Web Tokens)
/// 注意这个结构体多了连个 trait
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    /// 创建 token
    /// secret: 对应的口令
    pub fn crate_token(&self, secret: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(crate::error::Error::E("cuowu".to_string())),
        };
    }
}
