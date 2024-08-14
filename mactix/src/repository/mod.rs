mod dingtalk;
pub use dingtalk::get_ding_depts;

use super::config::APP_CONFIG;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn init_db() -> MySqlPool {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(
            APP_CONFIG
                .get()
                .unwrap()
                .database
                .get_connection_url()
                .as_str(),
        )
        .await
        .unwrap();
    pool
}
