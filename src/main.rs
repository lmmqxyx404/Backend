/*
 * @Author: Lmmqxyx
 * @Date: 2022-02-16 11:25:03
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-03-07 18:00:14
 * @FilePath: \backend\src\main.rs
 * @Description:
 */
// (depreciated) use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use backend::{
    controller::{img_verify_controller, sys_user_controller},
    middleware::auth_axum::Auth,
    service::CONTEXT,
};

use axum::routing::{get, post};
use axum::Router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. 大型后端项目需要首先记录日志

    // 2. 选择连接数据库
    // 切记，一定要连接数据库
    CONTEXT.init_pool().await;
    // 3. 启动路由服务
    // 3.1 首先创建服务器实例App::new()
    let app = Router::new()
        .route("/", get(|| async { "index" }))
        // 图片验证码
        .route("/admin/captcha", get(img_verify_controller::captcha))
        .route("/admin/sys_login", post(sys_user_controller::login))
        .route("/admin/sys_user_info", post(sys_user_controller::user_info))
        .route(
            "/admin/sys_user_detail",
            post(sys_user_controller::user_detail),
        )
        .layer(axum::middleware::from_fn(
            backend::middleware::auth_axum::auth,
        ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await
    /*
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
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
    */
}
