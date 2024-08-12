use actix_web::{web, App, HttpServer};
use mactix::router;
// 导入库 crate, tests才可以测试
// 详情可查看 https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html  ==> 二进制 crate 的集成测试
extern crate mactix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(mactix::greet))
            .route("/name/{name}", web::get().to(mactix::greet))
            .route("/health_check", web::get().to(mactix::health_check))
            // 使用宏
            .service(router::index1)
            // 使用宏 + scope
            .service(web::scope("/app").service(router::index2))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
