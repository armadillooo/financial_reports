use axum::{extract::Json, http::StatusCode};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;

use crate::common::JsonBuilder;

/// 汎用エラーメッセージ
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiError {
    pub message: &'static str,
}

// 汎用エラー型
pub type Rejection = (StatusCode, Json<Value>);

static INTERNAL_ERROR_RESPONSE: Lazy<Rejection> = Lazy::new(|| {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        JsonBuilder::new()
            .add(ApiError {
                message: "Internal server problem",
            })
            .build(),
    )
});
