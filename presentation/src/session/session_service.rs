use std::clone::Clone;

use crate::session::{SessionData, SessionId};

#[async_trait::async_trait]
pub trait SessionService: Clone {
    async fn find_or_create(&self, session_id: SessionId) -> anyhow::Result<SessionFromRequest>;
    async fn find(&self, session_id: SessionId) -> anyhow::Result<Option<SessionWithId>>;
    async fn create(&self) -> anyhow::Result<SessionWithId>;
    async fn save(&self, session: SessionData) -> anyhow::Result<SessionId>;
    async fn delete(&self, session: SessionData) -> anyhow::Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum SessionFromRequest {
    Found(SessionWithId),
    Refreshed(SessionWithId),
}

#[derive(Debug, PartialEq)]
pub struct SessionWithId {
    pub inner: SessionData,
    pub id: SessionId,
}
