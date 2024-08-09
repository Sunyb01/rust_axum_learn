// 导入库crate
extern crate web;
mod configs;
mod model;
mod router;
mod service;

use app;
use configs::APP_CONFIG;
use server;
use tokio::net::TcpListener;

// axum 这个库目前在linux环境上, 会出现内存泄漏的问题
// 主要原因是因为 linux gun libc;
// 可以使用 musl编译, 也可以使用三方的内存分配器
// 文章地址: https://rustcc.cn/article?id=61e5dd0a-45c6-4f09-8554-c6b7c01f202b
#[tokio::main]
async fn main() {
    web::web_lib_hello();
    app::hello_web();
    server::hello();
    let _ = APP_CONFIG.get_or_init(|| configs::load_properties());
    let app = router::init_and_register_router();
    let listener = TcpListener::bind(APP_CONFIG.get().unwrap().server.clone().get_addr())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
