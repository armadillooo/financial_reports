use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

pub type ApiResponse = (StatusCode, Json<Value>);

/// 汎用エラーメッセージ
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErrorResponse {
    pub message: &'static str,
}

pub fn internal_error() -> ApiResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(
            serde_json::to_value(ErrorResponse {
                message: "Internal server problem",
            })
            .unwrap(),
        ),
    )
}
