use thiserror::Error;

use domain::user::UserDomainError;

#[derive(Error, Debug)]
pub enum UserApplicationError {
    #[error("internal server error")]
    Disconnect,
    #[error("user is already exsist")]
    UserAlreadyExist,
    #[error("user not exist")]
    UserNotExist,
}

impl From<UserDomainError> for UserApplicationError {
    fn from(value: UserDomainError) -> Self {
        match value {
            UserDomainError::Disconnect(_) => Self::Disconnect,
            UserDomainError::UserAlreadyExist => Self::UserAlreadyExist,
            UserDomainError::UserNotFound => Self::UserNotExist,
        }
    }
}

pub type UserApplicationResult<T> = Result<T, UserApplicationError>;
