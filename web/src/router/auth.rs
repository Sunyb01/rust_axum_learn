use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    token_id: String,
    is_auth: bool,
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            token_id: String::from(""),
            is_auth: false,
        }
    }
}

impl Auth {
    fn new(token_id: String) -> Self {
        Auth {
            token_id: token_id,
            is_auth: true,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("token")
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) if token_is_valid(auth_header) => {
                Ok(Auth::new(auth_header.to_string()))
            }
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

fn token_is_valid(token: &str) -> bool {
    return token == "sss";
}
