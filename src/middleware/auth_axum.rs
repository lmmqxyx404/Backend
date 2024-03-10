use axum::{
    extract::Request,
    http,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    domain::vo::{jwt::JWT_Token, RespVO},
    service::CONTEXT,
};

use super::auth::{check_auth, checked_token, is_white_list_api};

use crate::error::Error;

pub struct Auth;

// 鉴权中间件
pub async fn auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    if !CONTEXT.config.debug {
        if !is_white_list_api(&path) {
            if let Ok(token) = get_token(&request.headers()) {
                if let Some(token) = token_is_valid(&token) {
                    let now = rbatis::rbdc::DateTime::now().unix_timestamp() as usize;
                    if (token.exp - now) < 100 {
                        // todo: add the attr support
                        let new_token = token.refresh(&CONTEXT.config.jwt_secret, 100).unwrap();
                        request.headers_mut().insert(
                            "access_token",
                            http::HeaderValue::from_str(&new_token).unwrap(),
                        );
                    }
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    let response = next.run(request).await;
    Ok(response)
}

fn token_is_valid(token: &str) -> Option<JWT_Token> {
    // 同步 fn 错误使用 async 注意看这个报错信息
    // expected opaque type `impl futures_util::Future<Output = std::result::Result<jwt::JWT_Token, error::Error>>`
    // found enum `std::result::Result<_, _>`
    match checked_token(token, "default") {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

fn get_token(h: &HeaderMap) -> Result<&str, Error> {
    Ok(h.get("access_token")
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or_default())
}
