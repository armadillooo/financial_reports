use async_trait::async_trait;

use super::session_data::SessionData;

#[async_trait]
pub trait SessionRepository {
    type Data: SessionData;

    async fn find(&self, cookie_value: &str) -> anyhow::Result<Option<Self::Data>>;
    async fn save(&self, session: Self::Data) -> anyhow::Result<String>;
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()>;
}
