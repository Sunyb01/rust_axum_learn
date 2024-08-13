use actix_web::{web, App, HttpServer};
use mactix::router;
// 导入库 crate, tests才可以测试
// 详情可查看 https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html  ==> 二进制 crate 的集成测试
extern crate mactix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mactix::config::init_config();
    let pool = mactix::repository::init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mactix::AppState { db: pool.clone() }))
            // 不要在service后面使用route, 会不生效
            .route("/", web::get().to(mactix::greet))
            .route("/name/{name}", web::get().to(mactix::greet))
            .route("/health_check", web::get().to(mactix::health_check))
            .route("/subscriptions", web::post().to(mactix::subscribe))
            .route("/index/index3", web::post().to(mactix::index3))
            .route("/index/index4", web::post().to(mactix::index4))
            // 使用宏
            .service(router::index1)
            // 使用宏 + scope
            .service(web::scope("/index").service(router::index2))
            .service(web::scope("persistence").service(mactix::persistence1))
    })
    .bind(
        mactix::config::APP_CONFIG
            .get()
            .unwrap()
            .server
            .clone()
            .get_addr(),
    )?
    .run()
    .await
}
