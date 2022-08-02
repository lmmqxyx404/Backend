use actix_web::{web, Responder};

use crate::domain::dto::sign_in::SignDTO;
use crate::domain::vo::RespVO;
use crate::error::Error;

/// 用户登录
pub async fn login(arg: web::Json<SignDTO>) -> impl Responder {
    // let login_vo = Err("empty account");
    return RespVO::<()>::from_error(&Error::from("empty account"), "-1").resp_json();
}
