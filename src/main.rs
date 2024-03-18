mod configs;
mod model;
mod router;
mod service;

use configs::APP_CONFIG;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let _ = APP_CONFIG.get_or_init(|| configs::load_properties());
    let app = router::init_and_regist();
    let listener = TcpListener::bind(APP_CONFIG.get().unwrap().server.clone().get_addr())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
