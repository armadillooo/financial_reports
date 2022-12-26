use thiserror::Error;

use super::UserId;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("user is already exsist: id={0:?}")]
    UserAlreadyExist(UserId),
    #[error("user not exist: id={0:?}")]
    UserNotFound(UserId),
}

pub type UserDomainResult<T> = Result<T, UserDomainError>;
