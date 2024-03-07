use std::{
    future::{ready, Ready},
    rc::Rc,
};

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use futures_util::future::LocalBoxFuture;

use crate::{domain::vo::RespVO, service::CONTEXT};

use super::auth::{check_auth, checked_token, is_white_list_api};

pub struct Auth;

// 鉴权中间件
pub async fn auth(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    if !CONTEXT.config.debug {
        todo!()
    }
    let response = next.run(request).await;
    Ok(response)
}
