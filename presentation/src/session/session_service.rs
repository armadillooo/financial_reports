use std::clone::Clone;

use crate::session::{SessionData, SessionId, SessionResult};

#[async_trait::async_trait]
pub trait SessionService {
    async fn find_or_create(&self, session_id: SessionId) -> SessionResult<SessionFromRequest>;
    async fn find(&self, session_id: SessionId) -> SessionResult<Option<SessionWithId>>;
    async fn create(&self) -> SessionResult<SessionWithId>;
    async fn save(&self, session: SessionData) -> SessionResult<SessionId>;
    async fn delete(&self, session: SessionData) -> SessionResult<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionFromRequest {
    Found(SessionWithId),
    Created(SessionWithId),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionWithId {
    pub inner: SessionData,
    pub id: SessionId,
}
