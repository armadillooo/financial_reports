use async_trait::async_trait;

use crate::session::SessionData;

#[async_trait]
pub trait SessionRepository {
    async fn find(&self, cookie_value: &str) -> anyhow::Result<Option<SessionData>>;
    async fn save(&self, session: SessionData) -> anyhow::Result<String>;
    async fn delete(&self, session: SessionData) -> anyhow::Result<()>;
}
