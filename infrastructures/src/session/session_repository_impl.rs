use async_session::SessionStore;
use async_trait::async_trait;

use presentation::session::{
    SessionData, SessionError, SessionId, SessionRepository, SessionResult,
};

#[derive(Debug, Clone)]
pub struct SessionRepositoryImpl<T: SessionStore> {
    store: T,
}

impl<T: SessionStore> SessionRepositoryImpl<T> {
    /// コンストラクタ
    pub fn new(store: T) -> Self {
        Self { store }
    }
}

#[async_trait]
impl<T: SessionStore> SessionRepository for SessionRepositoryImpl<T> {
    /// Session削除
    async fn delete(&self, session_id: SessionId) -> SessionResult<()> {
        let session = self
            .find(session_id)
            .await?
            .ok_or(SessionError::SessionNotFound)?;
        let session_id = session.into();

        self.store
            .destroy_session(session_id)
            .await
            .map_err(|e| SessionError::Disconnect(e))
    }

    /// Session取得
    async fn find(&self, session_id: SessionId) -> SessionResult<Option<SessionData>> {
        if let Some(mut session) = self
            .store
            .load_session(session_id.to_string())
            .await
            .map_err(|e| SessionError::Disconnect(e))?
        {
            session.set_cookie_value(session_id.to_string());
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> SessionResult<Option<SessionId>> {
        let session_id = self
            .store
            .store_session(session.into())
            .await
            .map_err(|e| SessionError::Disconnect(e))?;

        Ok(session_id.map(|id| SessionId::new(id)))
    }
}
