/// 配置
///
use config::{Config, File};

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationProperties {
    pub database: Database,
    pub server: Server,
}

impl Default for ApplicationProperties {
    fn default() -> Self {
        ApplicationProperties {
            database: Database::default(),
            server: Server::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Database {
    url: String,
    port: i32,
    database: String,
    username: String,
    password: String,
}

impl Default for Database {
    fn default() -> Self {
        Database {
            url: String::from(""),
            port: 3306,
            database: String::from(""),
            username: String::from(""),
            password: String::from(""),
        }
    }
}

impl Database {
    pub fn get_connection_url(&self) -> String {
        format!("mysql://{}:{}@{}:{}/{}?allowMultiQueries=true&useUnicode=true&characterEncoding=UTF-8&serverTimezone=GMT%2b8", self.username, self.password, self.url, self.port, self.database)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    host: String,
    port: i32,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            host: String::from("localhost"),
            port: 8899,
        }
    }
}

impl Server {
    pub fn get_addr(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn get_config() -> ApplicationProperties {
    let settings = Config::builder()
        // 项目名/文件目录/文件名
        .add_source(File::with_name("mactix/src/config/application.toml"))
        .build()
        .expect("构建配置错误");
    settings
        .try_deserialize::<ApplicationProperties>()
        .expect("反序列化配置文件错误")
}

pub fn init_config() {
    APP_CONFIG.get_or_init(|| get_config());
}

pub static APP_CONFIG: OnceLock<ApplicationProperties> = OnceLock::new();
