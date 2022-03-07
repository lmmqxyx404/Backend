/*
 * @Author: Lmmqxyx
 * @Date: 2022-02-16 11:25:03
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-03-07 18:00:14
 * @FilePath: \backend\src\main.rs
 * @Description:
 */
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8001")?
        .run()
        .await
}
