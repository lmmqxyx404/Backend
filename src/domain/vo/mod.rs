use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use actix_web::HttpResponse;
const SUCCESS_CODE: &str = "SUCCESS";
const FAIL_CODE: &str = "FAIL";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(SUCCESS_CODE.to_string()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(FAIL_CODE.to_string()),
                data: None,
                msg: Some(arg.clone().err().unwrap().to_string()),
            }
        }
    }

    /// 转化为标准 RespVO
    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(SUCCESS_CODE.to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn resp_json(&self) -> HttpResponse {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content_Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

/// 实现 ToString trait 从而方便给 resp_json 方法使用
impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
