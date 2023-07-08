use crate::{domain::vo::jwt::JWT_Token, service::CONTEXT};

pub async fn checked_token(token: &str, path: &str) -> Result<JWT_Token, crate::error::Error> {
    let token = JWT_Token::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}
