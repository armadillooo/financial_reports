use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("internal server error")]
    Disconnect,
    #[error("user is already exsist")]
    UserAlreadyExist,
}

pub type UserDomainResult<T> = Result<T, UserDomainError>;
