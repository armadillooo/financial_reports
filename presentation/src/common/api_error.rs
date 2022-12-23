use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use applications::{
    company::CompanyQueryError, favorite::FavoriteApplicationError,
    portfolio::PortfolioApplicationError, stock::StockQueryError, users::UserApplicationError,
};

use crate::{auth::OICDError, session::SessionError};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    UserApplicationError(#[from] UserApplicationError),
    #[error(transparent)]
    FavoriteApplicationError(#[from] FavoriteApplicationError),
    #[error(transparent)]
    PortfolioApplicationError(#[from] PortfolioApplicationError),
    #[error(transparent)]
    CompanyQueryError(#[from] CompanyQueryError),
    #[error(transparent)]
    StockQueryError(#[from] StockQueryError),
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    OICDError(#[from] OICDError),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::UserApplicationError(e) => match e {
                UserApplicationError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                UserApplicationError::UserAlreadyExist => StatusCode::BAD_REQUEST,
                UserApplicationError::UserNotExist => StatusCode::NOT_FOUND,
            },
            ApiError::FavoriteApplicationError(e) => match e {
                FavoriteApplicationError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                FavoriteApplicationError::UserAlreadyExist => StatusCode::BAD_REQUEST,
                FavoriteApplicationError::UserNotFound => StatusCode::NOT_FOUND,
            },
            ApiError::PortfolioApplicationError(e) => match e {
                PortfolioApplicationError::PortfolioNotFound => StatusCode::NOT_FOUND,
                PortfolioApplicationError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                PortfolioApplicationError::UserNotFound => StatusCode::NOT_FOUND,
                PortfolioApplicationError::UserAlreadyExist => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::InvalidRangeOfDate { .. } => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::StockDataNotFound => StatusCode::NOT_FOUND,
            },
            ApiError::CompanyQueryError(e) => match e {
                CompanyQueryError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                CompanyQueryError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                CompanyQueryError::CompanyNotFound => StatusCode::NOT_FOUND,
            },
            ApiError::StockQueryError(e) => match e {
                StockQueryError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                StockQueryError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                StockQueryError::StockDataNotFound => StatusCode::NOT_FOUND,
                StockQueryError::InvalidRangeOfDate { .. } => StatusCode::BAD_REQUEST,
            },
            ApiError::SessionError(e) => match e {
                SessionError::Disconnect => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::ItemNotFound => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::ItemNotSaved => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::SessionIdNotSend => StatusCode::BAD_REQUEST,
                SessionError::SessionNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ApiError::OICDError(e) => match e {
                OICDError::InitializationError => StatusCode::INTERNAL_SERVER_ERROR,
                OICDError::NotRegisterdEmail => StatusCode::BAD_REQUEST,
                OICDError::VerifyError => StatusCode::BAD_REQUEST,
                OICDError::ItemNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };
        let message = self.to_string();
        let body = serde_json::json!({
            "error": {
                "message": message
            }
        });
        let res = (status_code, Json(body)).into_response();

        res
    }
}
