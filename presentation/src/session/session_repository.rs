use async_trait::async_trait;

use super::session_data::SessionData;

#[async_trait]
pub trait SessionRepository {
    type Data: SessionData;

    async fn save(&self, session: Self::Data) -> anyhow::Result<String>;
    async fn find(&self, session_id: &str) -> anyhow::Result<Option<Self::Data>>;
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()>;
}
