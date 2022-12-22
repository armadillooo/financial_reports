use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortfolioDomainError {
    #[error("internal server error")]
    Disconnect,
    #[error("portfolio not found")]
    PortfolioNotFound,
}

pub type PortfolioDomainResult<T> = Result<T, PortfolioDomainError>;