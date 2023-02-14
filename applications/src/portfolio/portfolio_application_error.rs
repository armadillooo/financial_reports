use chrono::NaiveDate;
use thiserror::Error;

use crate::stock::StockQueryError;
use domain::{portfolio::PortfolioDomainError, user::UserDomainError};

#[derive(Error, Debug)]
pub enum PortfolioApplicationError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("portfolio not found: id={0}")]
    PortfolioNotFound(String),
    #[error("user is already exsist: id={0}")]
    UserAlreadyExist(String),
    #[error("user not found: id={0}")]
    UserNotFound(String),
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: &'static str, value: String },
    #[error("invalid date parameter: {name}={value}")]
    InvalidRangeOfDate {
        name: &'static str,
        value: NaiveDate,
    },
    #[error("stock data not found: id={0}")]
    StockDataNotFound(String),
}

pub type PortfoliApplicationResult<T> = Result<T, PortfolioApplicationError>;

impl From<PortfolioDomainError> for PortfolioApplicationError {
    fn from(value: PortfolioDomainError) -> Self {
        match value {
            PortfolioDomainError::Disconnect(e) => Self::Disconnect(e),
        }
    }
}

impl From<UserDomainError> for PortfolioApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect(e) => Self::Disconnect(e),
            UserDomainError::UserAlreadyExist(user_id) => Self::UserAlreadyExist(user_id.into()),
            UserDomainError::UserNotFound(user_id) => Self::UserNotFound(user_id.into()),
        }
    }
}

impl From<StockQueryError> for PortfolioApplicationError {
    fn from(value: StockQueryError) -> Self {
        match value {
            StockQueryError::Disconnect(e) => Self::Disconnect(e),
            StockQueryError::InvalidParameter { name, value } => {
                Self::InvalidParameter { name, value }
            }
            StockQueryError::InvalidRangeOfDate { name, value } => {
                Self::InvalidRangeOfDate { name, value }
            }
            StockQueryError::StockDataNotFound(stock_id) => Self::StockDataNotFound(stock_id),
        }
    }
}
