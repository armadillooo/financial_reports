use std::clone::Clone;

use crate::session::SessionData;

#[async_trait::async_trait]
pub trait SessionService: Clone {
    async fn find_or_create(&self, cookie_value: String) -> anyhow::Result<SessionFromRequest>;
    async fn create(&self) -> anyhow::Result<SessionMetadata>;
    async fn save(&self, session: SessionData) -> anyhow::Result<String>;
    async fn delete(&self, session: SessionData) -> anyhow::Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum SessionFromRequest {
    Found(SessionMetadata),
    Created(SessionMetadata),
}

#[derive(Debug, PartialEq)]
pub struct SessionMetadata {
    pub inner: SessionData,
    pub cookie_value: String,
}
