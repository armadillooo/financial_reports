use thiserror::Error;

use crate::stock::StockQueryError;
use domain::{portfolio::PortfolioDomainError, users::UserDomainError};

#[derive(Error, Debug)]
pub enum PortfolioApplicationError {
    #[error(transparent)]
    FromDomain(#[from] PortfolioDomainError),
    #[error(transparent)]
    FromUserDomain(#[from] UserDomainError),
    #[error(transparent)]
    FromStockQuery(#[from] StockQueryError),
    #[error("market price data not found")]
    MarketPriceNotFound,
}

pub type PortfoliApplicationResult<T> = Result<T, PortfolioApplicationError>;
