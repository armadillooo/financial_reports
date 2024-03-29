use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::{auth::OICDError, session::SessionError};
use applications::{
    company::CompanyQueryError, favorite::FavoriteApplicationError,
    portfolio::PortfolioApplicationError, stock::StockQueryError, user::UserApplicationError,
};

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
        let code = match &self {
            ApiError::UserApplicationError(e) => match e {
                UserApplicationError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                UserApplicationError::UserAlreadyExist(_) => StatusCode::BAD_REQUEST,
                UserApplicationError::UserNotExist(_) => StatusCode::NOT_FOUND,
            },
            ApiError::FavoriteApplicationError(e) => match e {
                FavoriteApplicationError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                FavoriteApplicationError::UserAlreadyExist(_) => StatusCode::BAD_REQUEST,
                FavoriteApplicationError::UserNotFound(_) => StatusCode::NOT_FOUND,
            },
            ApiError::PortfolioApplicationError(e) => match e {
                PortfolioApplicationError::PortfolioNotFound(_) => StatusCode::NOT_FOUND,
                PortfolioApplicationError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                PortfolioApplicationError::UserNotFound(_) => StatusCode::NOT_FOUND,
                PortfolioApplicationError::UserAlreadyExist(_) => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::InvalidRangeOfDate { .. } => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                PortfolioApplicationError::StockDataNotFound(_) => StatusCode::NOT_FOUND,
            },
            ApiError::CompanyQueryError(e) => match e {
                CompanyQueryError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                CompanyQueryError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                CompanyQueryError::CompanyNotFound(_) => StatusCode::NOT_FOUND,
            },
            ApiError::StockQueryError(e) => match e {
                StockQueryError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                StockQueryError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
                StockQueryError::StockDataNotFound(_) => StatusCode::NOT_FOUND,
                StockQueryError::InvalidRangeOfDate { .. } => StatusCode::BAD_REQUEST,
            },
            ApiError::SessionError(e) => match e {
                SessionError::Disconnect(_) => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::ItemNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::SavingItemError => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::SessionNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
                SessionError::IntoSessionIdError => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ApiError::OICDError(e) => match e {
                OICDError::ParameterRequired { .. } => StatusCode::BAD_REQUEST,
                OICDError::VerifyError(_) => StatusCode::BAD_REQUEST,
                OICDError::EmailNotRegisterd => StatusCode::BAD_REQUEST,
                OICDError::AuthenticationRequired => StatusCode::UNAUTHORIZED,
            },
        };
        let message = if code == StatusCode::INTERNAL_SERVER_ERROR {
            "internal server error".to_string()
        } else {
            self.to_string()
        };
        let body = serde_json::json!({
            "error": {
                "message": message
            }
        });
        let res = (code, Json(body)).into_response();

        res
    }
}
