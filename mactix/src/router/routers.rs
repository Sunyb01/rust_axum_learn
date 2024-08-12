use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

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
