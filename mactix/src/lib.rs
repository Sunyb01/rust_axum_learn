//! 在Rust中, main.rs为二进制crate, lib.rs为库crate;
//! Rust中，main.rs文件主要是用来定义可执行程序的入口点，而不是存放公共函数的地方。
//! 通常，您会将公共函数放在lib.rs文件中，以便在整个库中重用，并在测试中使用。
//! 导入router mod
//!

pub mod config;
pub mod model;
pub mod pojo;
pub mod repository;
pub mod router;
pub mod common;

use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
}
