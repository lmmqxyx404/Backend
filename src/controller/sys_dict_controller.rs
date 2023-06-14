use actix_web::{web, Responder};

use crate::{
    domain::{
        dto::{
            dict::{DictAddDTO, DictEditDTO, DictPageDTO},
            IdDTO,
        },
        table::SysDict,
        vo::RespVO,
    },
    error::Error,
    service::CONTEXT,
};

/// 字典分页
pub async fn page(page: web::Json<DictPageDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.page(&page.0).await;

    // RespVO::<String>::from_error(&Error::from("access token is empty"), "-10").resp_json()
    RespVO::from_result(&data).resp_json()
}

/// 字典添加
pub async fn add(mut arg: web::Json<DictAddDTO>) -> impl Responder {
    match (&arg.name, &arg.code) {
        (Some(name), Some(code)) => {
            if arg.state.is_none() {
                arg.state = Some(1);
            }
            let res = SysDict::from(arg.0);
            let data = CONTEXT.sys_dict_service.add(&res).await;
            CONTEXT.sys_dict_service.update_cache().await;
            RespVO::from_result(&data).resp_json()
        }
        (_, _) => RespVO::<u64>::from_error(&Error::from("dictionary data error"), "").resp_json(),
    }
}

/// 字典修改
pub async fn update(mut arg: web::Json<DictEditDTO>) -> impl Responder {
    let data = CONTEXT.sys_dict_service.edit(&arg.0).await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
    // RespVO::<String>::from_error(&Error::from("access token is empty"), "-10").resp_json()
}

/// 字典删除
pub async fn remove(mut arg: web::Json<IdDTO>) -> impl Responder {
    let data = CONTEXT
        .sys_dict_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    CONTEXT.sys_dict_service.update_cache().await;
    RespVO::from_result(&data).resp_json()
    // RespVO::<String>::from_error(&Error::from("access token is empty"), "-10").resp_json()
}
