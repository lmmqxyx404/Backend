use actix_web::{web, Responder};

use crate::{
    domain::{dto::dict::DictPageDTO, vo::RespVO},
    error::Error,
};

pub async fn page(page: web::Json<DictPageDTO>) -> impl Responder {
    RespVO::<String>::from_error(&Error::from("access token is empty"), "-10").resp_json()
}
