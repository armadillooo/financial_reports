use chrono::NaiveDate;
use thiserror::Error;

use crate::stock::StockQueryError;
use domain::{portfolio::PortfolioDomainError, users::UserDomainError};

#[derive(Error, Debug)]
pub enum PortfolioApplicationError {
    #[error("internal server error")]
    Disconnect,
    #[error("portfolio not found")]
    PortfolioNotFound,
    #[error("user is already exsist")]
    UserAlreadyExist,
    #[error("user not found")]
    UserNotFound,
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: String, value: String },
    #[error("start date exceeds end date")]
    InvalidRangeOfDate { start: NaiveDate, end: NaiveDate },
    #[error("stock data not found")]
    StockDataNotFound,
}

pub type PortfoliApplicationResult<T> = Result<T, PortfolioApplicationError>;

impl From<PortfolioDomainError> for PortfolioApplicationError {
    fn from(value: PortfolioDomainError) -> Self {
        match value {
            PortfolioDomainError::Disconnect => Self::Disconnect,
            PortfolioDomainError::PortfolioNotFound => Self::PortfolioNotFound,
        }
    }
}

impl From<UserDomainError> for PortfolioApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect => Self::Disconnect,
            UserDomainError::UserAlreadyExist => Self::UserAlreadyExist,
            UserDomainError::UserNotFound => Self::UserNotFound,
        }
    }
}

impl From<StockQueryError> for PortfolioApplicationError {
    fn from(value: StockQueryError) -> Self {
        match value {
            StockQueryError::Disconnect => Self::Disconnect,
            StockQueryError::InvalidParameter { name, value } => {
                Self::InvalidParameter { name, value }
            }
            StockQueryError::InvalidRangeOfDate { start, end } => {
                Self::InvalidRangeOfDate { start, end }
            }
            StockQueryError::StockDataNotFound => Self::StockDataNotFound,
        }
    }
}
