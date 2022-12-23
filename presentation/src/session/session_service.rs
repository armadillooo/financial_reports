use crate::session::{SessionId, SessionItem, SessionResult};

#[async_trait::async_trait]
pub trait SessionService {
    async fn find_or_create(&self, session_id: Option<SessionId>) -> SessionResult<SessionStatus>;
    async fn delete(&self, session_id: SessionId) -> SessionResult<()>;
    async fn item(&self, session_id: SessionId, key: &SessionItem) -> SessionResult<SessionItem>;
    async fn insert_item(&self, session_id: SessionId, item: SessionItem) -> SessionResult<()>;
    async fn remove_item(&self, session_id: SessionId, key: &SessionItem) -> SessionResult<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    /// セッション作成済み
    Found(SessionId),
    /// 新規作成
    Created(SessionId),
}

impl Into<SessionId> for SessionStatus {
    fn into(self) -> SessionId {
        match self {
            Self::Found(session_id) => session_id,
            Self::Created(session_id) => session_id,
        }
    }
}
