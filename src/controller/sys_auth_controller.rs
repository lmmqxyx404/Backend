use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
// use actix_web::{web, Responder};
use axum::response::IntoResponse;
use axum::Json;

use crate::service::CONTEXT;

///检测token以及path 是否有效且允许访问
/// todo: to optimize the relative fn
pub async fn check(arg: Json<SysAuthDTO>) -> impl IntoResponse {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0).await;
    RespVO::from_result(&Ok("uncompleted".to_string())).json()
}
