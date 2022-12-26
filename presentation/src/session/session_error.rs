use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("item not found: key={0:?}")]
    ItemNotFound(String),
    #[error("failed to save item")]
    SavingItemError,
    #[error("sessin id not send")]
    SessionIdRequired,
    #[error("sessin not found")]
    SessionNotFound,
    #[error("cannot get id from the session")]
    IntoSessionIdError,
}

pub type SessionResult<T> = Result<T, SessionError>;
