use thiserror::Error;

#[derive(Error, Debug)]
pub enum OICDError {
    #[error("internal server error")]
    InitializationError,
    #[error("email address was not registerd")]
    NotRegisterdEmail,
    #[error("authentication failed")]
    VerifyError,
    #[error("item not found")]
    ItemNotFound,
}

pub type OICDResult<T> = Result<T, OICDError>;
