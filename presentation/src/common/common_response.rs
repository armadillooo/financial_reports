use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiError {
    pub message: &'static str,
}
