use thiserror::Error;

use domain::user::UserDomainError;

#[derive(Error, Debug)]
pub enum UserApplicationError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("user is already exsist: id={0}")]
    UserAlreadyExist(String),
    #[error("user not exist: id={0}")]
    UserNotExist(String),
}

impl From<UserDomainError> for UserApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect(e) => Self::Disconnect(e),
            UserDomainError::UserAlreadyExist(user_id) => {
                Self::UserAlreadyExist(user_id.into())
            }
            UserDomainError::UserNotFound(user_id) => Self::UserNotExist(user_id.into()),
        }
    }
}

pub type UserApplicationResult<T> = Result<T, UserApplicationError>;
