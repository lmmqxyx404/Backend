use actix_web::{web, Responder};

use crate::{
    domain::{dto::res::ResPageDTO, vo::RespVO},
    service::CONTEXT,
};

pub async fn page(page: web::Json<ResPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_res_service.page(&page.0).await;
    RespVO::from_result(&data).resp_json()
}
