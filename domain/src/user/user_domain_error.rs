use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("user is already exsist")]
    UserAlreadyExist,
    #[error("user not exist")]
    UserNotFound,
}

pub type UserDomainResult<T> = Result<T, UserDomainError>;
