/*
 * @Author: Lmmqxyx
 * @Date: 2022-02-16 11:25:03
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-03-07 18:00:14
 * @FilePath: \backend\src\main.rs
 * @Description:
 */
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use backend::{
    controller::{img_verify_controller, sys_user_controller},
    middleware::auth_actix::Auth,
    service::CONTEXT,
};
async fn index() -> impl Responder {
    HttpResponse::Ok().body("[backend] Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. 大型后端项目需要首先记录日志

    // 2. 选择连接数据库
    // 切记，一定要连接数据库
    CONTEXT.init_pool().await;
    // 3. 启动路由服务
    // 3.1 首先创建服务器实例App::new()
    HttpServer::new(|| {
        App::new()
            .wrap(Auth{})
            .route("/", web::get().to(index))
            // 验证码路由接口
            .route(
                "/admin/captcha",
                web::get().to(img_verify_controller::captcha),
            )
            // 登录接口，实现登录功能
            .route(
                "/admin/sys_login",
                web::post().to(sys_user_controller::login),
            )
            // 获取用户信息接口
            .route(
                "/admin/sys_user_info",
                web::post().to(sys_user_controller::user_info),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
