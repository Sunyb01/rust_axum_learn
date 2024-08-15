use super::response::ResponseResult;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("An internal error occurred. Please try again later.")]
    InternalError,

    #[error("this is test")]
    TestError,

    #[error("this is test, id: {id}, msg: {msg}")]
    TestError2 { id: i32, msg: String },

    #[error("No login, uid: {uid}, uname: {}", uname)]
    UnLogin { uid: i32, uname: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&ResponseResult::fail(200, self.to_string())).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::TestError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UnLogin { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::TestError2 { id, .. } => {
                println!("id: {}", id);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
