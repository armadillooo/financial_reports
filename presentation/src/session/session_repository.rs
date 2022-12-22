use async_trait::async_trait;

use crate::session::{SessionData, SessionResult, SessionId};

#[async_trait]
pub trait SessionRepository {
    async fn find(&self, session_id: SessionId) -> SessionResult<Option<SessionData>>;
    async fn save(&self, session: SessionData) -> SessionResult<SessionId>;
    async fn delete(&self, session_id: SessionId) -> SessionResult<()>;
}
