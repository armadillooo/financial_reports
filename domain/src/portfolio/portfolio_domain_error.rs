use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortfolioDomainError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
}

pub type PortfolioDomainResult<T> = Result<T, PortfolioDomainError>;
