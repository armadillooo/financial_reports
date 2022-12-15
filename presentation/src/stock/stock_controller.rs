use axum::{extract::Extension, Router, response::IntoResponse, routing::get};

use crate::{
    common::{ErrorResponse, JsonBuilder, Utility, UtilityImpl},
};

pub fn stock_cotroller() -> Router {
    Router::new().route("/stocks",get(get_stocks))
}

async fn get_stocks(Extension(utility): Extension<UtilityImpl>) -> impl IntoResponse {
    unimplemented!();
}
