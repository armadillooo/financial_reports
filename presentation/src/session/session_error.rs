use thiserror::Error;

use crate::session::SessionId;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("item not found: key={0}")]
    ItemNotFound(&'static str),
    #[error("failed to save item")]
    SavingItemError,
    #[error("session not found: id={0:?}")]
    SessionNotFound(SessionId),
    #[error("cannot get id from the session")]
    IntoSessionIdError,
}

pub type SessionResult<T> = Result<T, SessionError>;
