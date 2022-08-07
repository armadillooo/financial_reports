use crate::session::SessionData;

#[async_trait::async_trait]
pub trait SessionService {
    type Data: SessionData;

    async fn start_session(&self, user_id: String) -> anyhow::Result<String>;
    async fn delete_session(&self, session: Self::Data) -> anyhow::Result<()>;
    async fn load_session(&self, session_id: &str) -> anyhow::Result<Option<Self::Data>>;
}
