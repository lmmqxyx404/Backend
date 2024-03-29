use crate::domain::dto::sign_in::CaptchaDTO;
use crate::domain::vo::RespVO;
use crate::error::Error;
use crate::util::string::isEmptyString;
// use actix_web::{web, HttpResponse, Responder};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;

use axum::body::Body;
use axum::extract::Query;
use axum::response::{IntoResponse, Response};

/// 图形验证码接口
/// debug 模式下无论缓存是否连接成功，都返回图片，release 模式下会校验reddis 缓存
/// 请求时必须带上 account
/// http://localhost:8000/admin/captcha?account=18900000000
pub async fn captcha(arg: Query<CaptchaDTO>) -> impl IntoResponse {
    // 账户为空时，不输出验证码，并且报错
    if arg.account.is_empty() {
        // return RespVO::<()>::from_error(&Error::from("account is empty"), "-1").json();
        let resp = Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "json")
            .body(Body::from("todo: changed account_empty"))
            .unwrap();
        return resp;
    }

    let mut captcha = Captcha::new();
    captcha
        // add_chars(4) 验证码字符个数
        .add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        // view 图片尺寸
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let img_str = captcha.chars_as_string().to_lowercase();
    // 如果在debug 模式，那么控制台应该输出相关信息
    println!("{:?}", img_str);

    // todo: 将验证码信息传递到 context 中，使得登录接口可以验证用户传递过来的验证码字符串

    // 传输图片时，需要转换为 u8 放进body中
    let png = captcha.as_png().unwrap();
    let resp = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "no-cache")
        .header("Content-Type", "image/png")
        .body(Body::from(png))
        .unwrap();
    return resp.into();
}
