// use actix_web::{web, Responder};
use axum::response::IntoResponse;
use axum::Json;

use crate::domain::dto::role::{SysRoleResAddDTO, SysRoleResUpdateDTO};
use crate::domain::dto::{EmptyDTO, IdDTO};
use crate::domain::vo::RespVO;
use crate::error::Error;
use crate::{domain::dto::role::SysRoleResPageDTO, service::CONTEXT};

/// 角色分页
pub async fn page(arg: Json<SysRoleResPageDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_res_service.page(&arg.0).await;
    return RespVO::from_result(&vo).json();
    // return RespVO::<()>::from_error(&Error::from("access token"), "-10").json();
}

/// 角色（关联资源添加）
pub async fn add(arg: Json<SysRoleResAddDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_res_service.add(&arg.0).await;

    return RespVO::from_result(&vo).json();
}

/// 层级数据
pub async fn layer_top(arg: Json<EmptyDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_service.finds_layer().await;
    return RespVO::from_result(&vo).json();
    // return RespVO::<()>::from_error(&Error::from("access token"), "-10").json();
}

/// 角色修改
pub async fn update(arg: Json<SysRoleResUpdateDTO>) -> impl IntoResponse {
    let vo = CONTEXT.sys_role_res_service.edit(&arg.0).await;
    return RespVO::from_result(&vo).json();
    // return RespVO::<()>::from_error(&Error::from("access token"), "-10").json();
}

/// 角色删除
pub async fn remove(arg: Json<IdDTO>) -> impl IntoResponse {
    let role_id = arg.0.id.unwrap_or_default();
    let vo = CONTEXT.sys_role_res_service.remove_role(&role_id).await;
    return RespVO::from_result(&vo).json();
    // return RespVO::<()>::from_error(&Error::from("access token"), "-10").resp_json();
}
