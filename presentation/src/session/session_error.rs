use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("item not found")]
    ItemNotFound,
    #[error("item not saved")]
    ItemNotSaved,
    #[error("sessin id not send")]
    SessionIdNotSend,
    #[error("sessin not found")]
    SessionNotFound,
    #[error("cannot get id from the session")]
    IntoSessionIdError
}

pub type SessionResult<T> = Result<T, SessionError>;
