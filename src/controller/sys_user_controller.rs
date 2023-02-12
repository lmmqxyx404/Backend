use actix_web::{web, HttpRequest, Responder};

use crate::domain::dto::sign_in::SignDTO;
use crate::domain::dto::user::{UserAddDTO, UserEditDTO, UserRolePageDTO};
use crate::domain::vo::jwt::JWT_Token;
use crate::domain::vo::RespVO;
use crate::error::Error;

use crate::service::CONTEXT;

use crate::domain::dto::IdDTO;
/// 用户登录
pub async fn login(arg: web::Json<SignDTO>) -> impl Responder {
    // let login_vo = Err("empty account");
    // 之后要打印日志
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    println!("get info vo {:?}", vo);
    return RespVO::from_result(&vo).resp_json();
    // return RespVO::<()>::from_error(&Error::from("empty account"), "-1").resp_json();
}

/// 用户基础信息接口
pub async fn user_info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access-token");
    match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWT_Token::verify(&CONTEXT.config.jwt_secret, token);
            if token.is_err() {
                return RespVO::from_result(&token).resp_json();
            }
            let user_data = CONTEXT
                .sys_user_service
                .get_user_info_by_token(&token.unwrap())
                .await;
            // 后续修改
            // 已经修改
            RespVO::from_result(&user_data).resp_json()
        }
        _ => RespVO::<String>::from_error(&Error::from("access token is empty"), "-10").resp_json(),
    }
}

/// 用户详情接口  暂时未实现
pub async fn user_detail(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.detail(&arg.0).await;
    RespVO::from_result(&vo).resp_json()
}

/// 修改用户信息
pub async fn user_update(arg: web::Json<UserEditDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.edit_user_info(arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 添加用户
/// 完成（2023年2月12日21点09分）
pub async fn user_add(arg: web::Json<UserAddDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_service.add(arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除用户
/// 删除日志没有记录（2023年2月12日23点14分）
pub async fn user_remove(arg: web::Json<IdDTO>) -> impl Responder {
    let vo = CONTEXT
        .sys_user_service
        .remove(&arg.0.id.unwrap_or_default())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户分页
pub async fn user_page(arg: web::Json<UserRolePageDTO>) -> impl Responder {
    let vo = CONTEXT.sys_user_role_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}
