use thiserror::Error;

#[derive(Error, Debug)]
pub enum FavoriteDomainError {
    #[error("internal server error")]
    Disconnect,
}

pub type FavoriteDomainResult<T> = Result<T, FavoriteDomainError>;
