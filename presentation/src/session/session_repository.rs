use async_trait::async_trait;

use super::session_data::SessionData;

#[async_trait]
pub trait SessionRepository {
    async fn save(&self, session: SessionData) -> anyhow::Result<String>;
    async fn find(&self, session_id: &str) -> anyhow::Result<Option<SessionData>>;
    async fn delete(&self, session: SessionData) -> anyhow::Result<()>;
}
