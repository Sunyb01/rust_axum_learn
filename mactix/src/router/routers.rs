//! router

use actix_web::{
    get, post,
    web::{Data, Form, Json},
    HttpRequest, HttpResponse, Responder,
};

use crate::common::UserError;
use crate::AppState;
use crate::{pojo, repository};
use logs::{error, info};

pub fn routers_hello() {
    println!("hello, this is routers")
}

#[get("/index1")]
pub async fn index1() -> impl Responder {
    HttpResponse::Ok().body("this is index1")
}

#[get("/index2")]
pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("this is index2")
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn router_hello() -> impl Responder {
    routers_hello();
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

#[get("/persistence1")]
pub async fn persistence1(data: Data<AppState>) -> impl Responder {
    let depts = repository::get_ding_depts(data, 0, 10).await;
    HttpResponse::Ok().json(depts)
}

#[post("/log")]
pub async fn log_test() -> impl Responder {
    info!("this is info");
    error!("this is error");
    HttpResponse::Ok()
}

#[get("/error1")]
pub async fn error1() -> Result<String, UserError> {
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok(String::from("success!"))
}

fn do_thing_that_fails() -> Result<(), UserError> {
    Err(UserError::TestError)
}
