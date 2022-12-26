use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("item not found")]
    ItemNotFound,
    #[error("item not saved")]
    SavingItemError,
    #[error("sessin id not send")]
    SessionIdRequired,
    #[error("sessin not found")]
    SessionNotFound,
    #[error("cannot get id from the session")]
    IntoSessionIdError,
}

pub type SessionResult<T> = Result<T, SessionError>;
