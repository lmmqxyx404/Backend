use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use actix_web::{web, Responder};

use crate::service::CONTEXT;

pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
    let r = CONTEXT.sys_auth_service.check_auth(arg.0);
    RespVO::from_result(&Ok("uncompleted".to_string())).resp_json()
}
