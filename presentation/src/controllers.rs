use axum::{routing::get, Router};

use crate::{auth::auth_controller, stock::stock_cotroller};

pub fn controllers() -> Router {
    Router::new()
        .nest("/api", auth_controller())
        .merge(Router::new().route("/", get(|| async { "hello" })))
}
