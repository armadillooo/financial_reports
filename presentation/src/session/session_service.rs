use std::clone::Clone;

use crate::session::{SessionId, SessionItemKey, SessionResult};

#[async_trait::async_trait]
pub trait SessionService {
    async fn find_or_create(&self, session_id: Option<SessionId>) -> SessionResult<SessionStatus>;
    async fn delete(&self, session_id: &SessionId) -> SessionResult<()>;
    async fn item<T>(
        &self,
        session_id: &SessionId,
        item_key: &SessionItemKey<T>,
    ) -> SessionResult<Option<T>>;
    async fn insert_item<T>(
        &self,
        session_id: &SessionId,
        item_key: &SessionItemKey<T>,
    ) -> SessionResult<()>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    /// セッション作成済み
    Found(SessionId),
    /// 新規作成
    Created(SessionId),
}
