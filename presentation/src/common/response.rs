use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

pub type ApiResponse = (StatusCode, Json<Value>);

#[derive(Debug, Clone, Serialize)]
pub struct ResponseJson {
    message: String,
}
