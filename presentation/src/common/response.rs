use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseJson {
    message: String,
}
