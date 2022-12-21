use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("internal server error")]
    Disconnect,
    #[error("user is already exsist")]
    UserAlreadyExist,
    #[error("user not exist")]
    UserNotExist,
}

pub type UserDomainResult<T> = Result<T, UserDomainError>;
