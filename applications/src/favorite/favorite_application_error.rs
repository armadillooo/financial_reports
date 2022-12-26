use thiserror::Error;

use domain::{favorite::FavoriteDomainError, user::UserDomainError};

#[derive(Error, Debug)]
pub enum FavoriteApplicationError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("user not found: id={0:?}")]
    UserNotFound(String),
    #[error("user is already exsist: id={0:?}")]
    UserAlreadyExist(String),
}

pub type FavoriteApplicationResult<T> = Result<T, FavoriteApplicationError>;

impl From<FavoriteDomainError> for FavoriteApplicationError {
    fn from(value: FavoriteDomainError) -> Self {
        match value {
            FavoriteDomainError::Disconnect(e) => Self::Disconnect(e),
        }
    }
}

impl From<UserDomainError> for FavoriteApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect(e) => Self::Disconnect(e),
            UserDomainError::UserAlreadyExist(user_id) => {
                Self::UserAlreadyExist(user_id.to_string())
            }
            UserDomainError::UserNotFound(user_id) => Self::UserNotFound(user_id.to_string()),
        }
    }
}
