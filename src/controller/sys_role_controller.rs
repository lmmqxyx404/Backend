use actix_web::{web, Responder};

use crate::domain::dto::role::SysRoleResAddDTO;
use crate::domain::vo::RespVO;
use crate::error::Error;
use crate::{domain::dto::role::SysRoleResPageDTO, service::CONTEXT};

/// 角色分页
pub async fn page(arg: web::Json<SysRoleResPageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
    // return RespVO::<()>::from_error(&Error::from("access token"), "-10").resp_json();
}

/// 角色（关联资源添加）
pub async fn add(arg: web::Json<SysRoleResAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_role_res_service.add(&arg.0).await;

    return RespVO::<()>::from_error(&Error::from("access token"), "-10").resp_json();
}
