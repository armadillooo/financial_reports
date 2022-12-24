use thiserror::Error;

use domain::{favorite::FavoriteDomainError, user::UserDomainError};

#[derive(Error, Debug)]
pub enum FavoriteApplicationError {
    #[error("internal server error")]
    Disconnect,
    #[error("user not found")]
    UserNotFound,
    #[error("user already exist")]
    UserAlreadyExist,
}

pub type FavoriteApplicationResult<T> = Result<T, FavoriteApplicationError>;

impl From<FavoriteDomainError> for FavoriteApplicationError {
    fn from(value: FavoriteDomainError) -> Self {
        match value {
            FavoriteDomainError::Disconnect => Self::Disconnect,
        }
    }
}

impl From<UserDomainError> for FavoriteApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect => Self::Disconnect,
            UserDomainError::UserAlreadyExist => Self::UserAlreadyExist,
            UserDomainError::UserNotFound => Self::UserNotFound,
        }
    }
}
