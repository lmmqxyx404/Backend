use actix_web::{web, HttpRequest, Responder};

use crate::domain::dto::sign_in::SignDTO;
use crate::domain::vo::RespVO;
use crate::error::Error;

use crate::service::CONTEXT;

/// 用户登录
pub async fn login(arg: web::Json<SignDTO>) -> impl Responder {
    // let login_vo = Err("empty account");
    // 之后要打印日志
    let vo = CONTEXT.sys_user_service.sign_in(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
    // return RespVO::<()>::from_error(&Error::from("empty account"), "-1").resp_json();
}

/// 用户基础信息接口
pub async fn user_info(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access-token");
    match token {
        Some(token) => {
            let token_token = token.to_str().unwrap_or("");
            // 后续修改
            return RespVO::<()>::from_error(&Error::from("access token"), "-10").resp_json();
        }
        _ => return RespVO::<()>::from_error(&Error::from("access token"), "-10").resp_json(),
    }
}

/* 用户详情接口  暂时未实现
pub async fn user_detail(arg: web::Json<IdDTO>) -> impl Responder{

}
*/
