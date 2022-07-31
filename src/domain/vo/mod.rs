use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Error;

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
}
