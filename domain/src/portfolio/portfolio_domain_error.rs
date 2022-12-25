use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortfolioDomainError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("portfolio not found")]
    PortfolioNotFound,
}

pub type PortfolioDomainResult<T> = Result<T, PortfolioDomainError>;
