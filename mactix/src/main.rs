//!
//! 入口
//!
use actix_web::{middleware::Logger, web, App, HttpServer};
use log4rs;
use mactix::router;
// 导入库 crate, tests才可以测试
// 详情可查看 https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html  ==> 二进制 crate 的集成测试
extern crate mactix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志, mactix/log4rs.yaml ==> mactix下的log4rs.yaml
    log4rs::init_file("mactix/log4rs.yaml", Default::default()).unwrap();
    // 初始化配置
    mactix::config::init_config();
    // 初始化数据库
    let pool = mactix::repository::init_db().await;
    // 构建服务
    HttpServer::new(move || {
        App::new()
            // 开启默认日志
            .wrap(Logger::default())
            // 设置app_state
            .app_data(web::Data::new(mactix::AppState { db: pool.clone() }))
            // 不要在service后面使用route, 会不生效
            .route("/", web::get().to(router::greet))
            .route("/name/{name}", web::get().to(router::greet))
            .route("/health_check", web::get().to(router::health_check))
            .route("/subscriptions", web::post().to(router::subscribe))
            .route("/index/index3", web::post().to(router::index3))
            .route("/index/index4", web::post().to(router::index4))
            // 使用宏
            .service(router::index1)
            // 使用宏 + scope
            .service(web::scope("/index").service(router::index2))
            .service(web::scope("persistence").service(router::persistence1))
            .service(web::scope("/log").service(router::log_test))
            .service(web::scope("/err").service(router::error1))
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
