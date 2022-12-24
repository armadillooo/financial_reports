use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

/// すべてのAPIに共通なレスポンス型
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// コンストラクタ
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
