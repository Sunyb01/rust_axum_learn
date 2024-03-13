use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseResult<T> {
    code: i32,
    msg: String,
    data: T,
}

impl<T> ResponseResult<T> {
    pub fn new(code: i32, msg: String, data: T) -> ResponseResult<T> {
        ResponseResult { code, msg, data }
    }

    pub fn success(data: T) -> ResponseResult<T> {
        return Self::new(200, String::from("success"), data);
    }
}
