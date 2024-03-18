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
    domain::table,
    middleware::auth_axum::Auth,
    service::CONTEXT,
};

use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post},
};
use axum::{
    http::{self, Response, StatusCode},
    Router,
};

async fn global_options_middleware(req: Request, next: Next) -> impl IntoResponse {
    if req.method() == http::Method::OPTIONS {
        // 返回统一的OPTIONS响应
        Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
            .header(
                "Access-Control-Allow-Headers",
                "X-PINGOTHER, Content-Type, Authorization",
            )
            .body(Body::empty())
            .unwrap()
    } else {
        // 对于非OPTIONS请求，继续传递到下一个中间件或路由处理器
        next.run(req).await
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. 大型后端项目需要首先记录日志

    // 2. 选择连接数据库
    // 切记，一定要连接数据库
    CONTEXT.init_pool().await;
    table::sync_tables(&CONTEXT.rbatis).await;
    table::sync_tables_data(&CONTEXT.rbatis).await;
    // 3. 启动路由服务
    // 3.1 首先创建路由实例App::new()
    let app = Router::new()
        .route("/", get(|| async { "BACKEND START index ppp" }))
        // 图片验证码
        .route("/admin/captcha", get(img_verify_controller::captcha))
        // 登录接口，实现登录功能
        .route("/admin/sys_login", post(sys_user_controller::login))
        .route("/admin/sys_user_info", post(sys_user_controller::user_info))
        .route(
            "/admin/sys_user_detail",
            post(sys_user_controller::user_detail),
        )
        .route("/admin/sys_user_add", post(sys_user_controller::user_add))
        .layer(axum::middleware::from_fn(global_options_middleware))
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
