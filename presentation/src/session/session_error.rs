use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session server error")]
    Disconnect,
    #[error("item not found")]
    ItemNotFound,
    #[error("item not saved")]
    ItemNotSaved,
    #[error("sessin id not send")]
    SessionIdNotSend,
    #[error("sessin not found")]
    SessionNotFound,
}

pub type SessionResult<T> = Result<T, SessionError>;
