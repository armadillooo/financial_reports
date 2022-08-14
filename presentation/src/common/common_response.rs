use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

/// 汎用エラーメッセージ
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiError {
    pub message: &'static str,
}

// 汎用エラー型
pub type Rejection = (StatusCode, Json<Value>);
