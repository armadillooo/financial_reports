use thiserror::Error;

#[derive(Error, Debug)]
pub enum FavoriteDomainError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
}

pub type FavoriteDomainResult<T> = Result<T, FavoriteDomainError>;
