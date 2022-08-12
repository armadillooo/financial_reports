use std::clone::Clone;

use crate::session::SessionData;

#[async_trait::async_trait]
pub trait SessionService: Clone {
    async fn find_or_create(&self, cookie_value: &str) -> anyhow::Result<SessionFromRequest>;
    async fn save(&self, session: SessionData) -> anyhow::Result<String>;
    async fn delete(&self, session: SessionData) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum SessionFromRequest {
    Found(SessionData),
    Created(CreatedSession),
}

#[derive(Debug)]
pub struct CreatedSession {
    pub session: SessionData,
    pub cookie_value: String,
}
