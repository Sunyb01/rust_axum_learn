/// 相应
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> ResponseResult<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }

    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: data,
        }
    }
}

impl ResponseResult<()> {
    pub fn fail(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: (),
        }
    }
}
