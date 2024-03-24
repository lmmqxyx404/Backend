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
    pub permissions: Vec<String>,
    /// 角色id集合
    pub role_ids: Vec<String>,
    /// 过期时间
    /// todo: change the name
    pub exp: usize,
}

impl JWT_Token {
    /// 创建 token
    /// secret: 对应的口令
    pub fn create_token(&self, secret: &str) -> Result<String> {
        match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(crate::error::Error::E("cuowu".to_string())),
        }
    }
    /// verify token
    /// secret: 对应的口令
    pub fn verify(secret: &str, token: &str) -> Result<JWT_Token> {
        let validation = Validation::default();
        return match decode::<JWT_Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    return Err(crate::error::Error::E("InvalidToken".to_string()))
                }
                ErrorKind::InvalidIssuer => {
                    return Err(crate::error::Error::E("InvalidIssuer".to_string()))
                }
                _ => return Err(crate::error::Error::E("Unknmown Token Error".to_string())),
            },
        };
    }

    /// 刷新token 时间为传入的双倍
    pub fn refresh(&self, secret: &str, jwt_exp: usize) -> Result<String> {
        let mut jwt = self.clone();
        jwt.exp = jwt.exp + jwt.exp;
        jwt.create_token(&secret)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rbatis::rbdc::types::datetime::DateTime;

    #[test]
    fn test_jwt() {
        let tt = JWT_Token {
            id: "1".to_string(),
            account: "189".to_string(),
            permissions: vec![],
            role_ids: vec![],
            exp: DateTime::now().unix_timestamp_millis() as usize,
        };
        let token = tt.create_token("ssss").unwrap();
        println!("{:?}", token);
        assert_eq!(JWT_Token::verify("ssss", &token).unwrap(), tt);
    }
}
