//! 在Rust中, main.rs为二进制crate, lib.rs为库crate;
//! Rust中，main.rs文件主要是用来定义可执行程序的入口点，而不是存放公共函数的地方。
//! 通常，您会将公共函数放在lib.rs文件中，以便在整个库中重用，并在测试中使用。
//! 导入router mod
//!

pub mod config;
pub mod pojo;
pub mod repository;
pub mod router;

use actix_web::{
    web::{Data, Form, Json},
    HttpRequest, HttpResponse, Responder,
};

use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn router_hello() -> impl Responder {
    router::routers_hello();
    HttpResponse::Ok()
}

pub async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn index3(info: Json<pojo::Info>) -> String {
    format!("Welcome {}!", info.username)
}

pub async fn index4(info: Form<pojo::Info>) -> String {
    format!("Welcome {} By from!", info.username)
}

pub async fn persistence1(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}
