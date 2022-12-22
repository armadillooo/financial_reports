use thiserror::Error;

#[derive(Error, Debug)]
pub enum OICDError {
    #[error("internal server error")]
    InitializationError,
    #[error("email address was not registerd")]
    NotRegisterdEmail,
    #[error("authentication failed")]
    VerifyError,
}

pub type OICDResult<T> = Result<T, OICDError>;
