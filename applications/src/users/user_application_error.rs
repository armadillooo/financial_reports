use thiserror::Error;

use domain::users::UserDomainError;

#[derive(Error, Debug)]
pub enum UserApplicationError {
    #[error(transparent)]
    FromDomain(#[from] UserDomainError)
}

pub type UserApplicationResult<T> = Result<T, UserApplicationError>;
