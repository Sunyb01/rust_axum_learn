use config::{Config, File};
/// 配置读取
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
#[warn(dead_code)]
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
            url: String::from("http://localhost"),
            port: 3306,
            database: String::from(""),
            username: String::from(""),
            password: String::from(""),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
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

pub fn load_properties() -> ApplicationProperties {
    let config = Config::builder()
        .add_source(File::with_name("web/config.toml"))
        .build()
        .expect("构建配置错误");

    let config: ApplicationProperties = config.try_deserialize().expect("反序列化配置文件错误");
    config
}

pub static APP_CONFIG: OnceLock<ApplicationProperties> = OnceLock::new();
