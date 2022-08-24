use axum::{routing::get, Router};

use crate::auth::auth_controller;

pub fn controllers() -> Router {
    Router::new()
        .nest("/api", auth_controller())
        .merge(Router::new().route("/", get(|| async { "hello" })))
}
