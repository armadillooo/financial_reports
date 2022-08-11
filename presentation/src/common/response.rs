use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ResponseJson {
    message: String,
}
