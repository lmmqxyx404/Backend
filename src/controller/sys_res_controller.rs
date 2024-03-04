// use actix_web::{web, Responder};
use axum::Json;
use axum::response::IntoResponse;

use crate::{
    domain::{dto::permission::ResPageDTO, vo::RespVO},
    service::CONTEXT,
};

pub async fn page(page: Json<ResPageDTO>) -> impl IntoResponse {
    let data = CONTEXT.sys_res_service.page(&page.0).await;
    RespVO::from_result(&data).json()
}
