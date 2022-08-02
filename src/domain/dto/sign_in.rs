use serde::{Deserialize, Serialize};
/// 验证码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaDTO {
    pub account: Option<String>,
}

/// 登录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignDTO{}