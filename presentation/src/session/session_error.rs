use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session server error")]
    Disconnect,
}

pub type SessionResult<T> = Result<T, SessionError>;
