use super::auth::Auth;
use crate::configs::APP_CONFIG;
use crate::model::ResponseResult;
use axum::{extract::Query, middleware::Next, routing::get, Json, Router};
use std::collections::HashMap;

pub fn init_and_register_router() -> Router {
    return Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/database",
            get(|| async {
                Json(ResponseResult::success(
                    APP_CONFIG.get().unwrap().database.clone(),
                ))
            }),
        )
        .route("/test/auth", get(test_auth))
        .layer(axum::middleware::from_extractor::<Auth>())
        .layer(axum::middleware::from_fn(|req, next: Next| async {
            println!("this is one ");
            next.run(req).await
        }))
        .layer(axum::middleware::from_fn(|req, next: Next| async {
            println!("this is two ");
            next.run(req).await
        }))
        .layer(axum::middleware::from_fn(|req, next: Next| async {
            println!("this is three ");
            next.run(req).await
        }));
}

async fn test_auth(
    auth: Auth,
    Query(params): Query<HashMap<String, String>>,
) -> Json<ResponseResult<Auth>> {
    println!("{:?}", auth);
    println!("{:?}", params);
    return Json(ResponseResult::success(auth));
}
