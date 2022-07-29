use actix_web::{web, HttpResponse, Responder};
use captcha::filters::{Dots, Noise, Wave};
use captcha::Captcha;

/// 图形验证码接口
/// debug 模式下无论缓存是否连接成功，都返回图片，release 模式下会校验reddis 缓存
/// 请求时必须带上 account
pub async fn captcha() -> impl Responder {


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

    // 将验证码信息传递到 context 中，使得登录接口可以验证用户传递过来的验证码字符串

    // 传输图片时，需要转换为 u8 放进body中
    let png = captcha.as_png().unwrap();
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .content_type("image/png")
        .body(png)
}
