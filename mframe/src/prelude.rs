//! Prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct W<T>(pub T);

// 自定义别名
pub use std::format as f;
