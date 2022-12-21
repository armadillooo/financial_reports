use thiserror::Error;

use domain::{favorite::FavoriteDomainError, users::UserDomainError};

#[derive(Error, Debug)]
pub enum FavoriteApplicationError {
    #[error(transparent)]
    FromDomain(#[from] FavoriteDomainError),
    #[error(transparent)]
    FromUserDomain(#[from] UserDomainError),
}

pub type FavoriteApplicationResult<T> = Result<T, FavoriteApplicationError>;
