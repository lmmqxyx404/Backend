use serde::{Deserialize, Serialize};
/// 验证码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaDTO {
    pub account: Option<String>,
}

/// 登录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignDTO {
    pub account: String,
    pub password: String,
    /// 验证码可以是短信验证码，图片验证码，二维码验证码
    pub vcode: String,
}
