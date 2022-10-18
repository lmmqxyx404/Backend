use crate::domain::dto::auth::SysAuthDTO;
use crate::domain::vo::RespVO;
use actix_web::{web, Responder};

use crate::service::{CONTEXT};

pub async fn check(arg: web::Json<SysAuthDTO>) -> impl Responder {
    let r=CONTEXT.
    RespVO::from_result(&Ok("uncompleted".to_string())).resp_json()
}
