use async_trait::async_trait;

use crate::session::{SessionData, SessionResult};

#[async_trait]
pub trait SessionRepository {
    async fn find(&self, session_id: String) -> SessionResult<Option<SessionData>>;
    async fn save(&self, session: SessionData) -> SessionResult<String>;
    async fn delete(&self, session: SessionData) -> SessionResult<()>;
}
