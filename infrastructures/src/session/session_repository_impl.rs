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
        let Some(session) = self
            .find(session_id.clone())
            .await?
            else {
                return Ok(())
            };
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
            .load_session(session_id.clone().into())
            .await
            .map_err(|e| SessionError::Disconnect(e))?
        {
            session.set_cookie_value(session_id.into());
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> SessionResult<SessionId> {
        let session_id = self
            .store
            .store_session(session.into())
            .await
            .map_err(|e| SessionError::Disconnect(e))?
            .ok_or(SessionError::IntoSessionIdError)?;

        Ok(SessionId::new(session_id))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use async_session::MemoryStore;

    use presentation::session::{SessionData, SessionRepository, SessionError, SessionId};

    use super::SessionRepositoryImpl;

    fn setup() -> impl SessionRepository {
        let repo = SessionRepositoryImpl::new(MemoryStore::new());

        repo
    }

    #[tokio::test]
    async fn save_session_success() -> anyhow::Result<()> {
        let repo = setup();
        let session = SessionData::new();
        let id = repo.save(session).await?;
        let saved = repo
            .find(id.clone())
            .await?
            .ok_or(anyhow!("session is not saved"))?;

        assert!(saved.into_session_id()? == id);

        Ok(())
    }

    #[tokio::test]
    async fn find_noexist_session_return_err() -> anyhow::Result<()> {
        let repo = setup();
        let Err(SessionError::SessionNotFound(_)) = repo.find(SessionId::new("aaaaa".to_string())).await else {
            return Err(anyhow!("unexpected find session result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_success() -> anyhow::Result<()> {
        let repo = setup();

        Ok(())
    }

    #[tokio::test]
    async fn delete_no_exist_session_return_ok() -> anyhow::Result<()> {
        let repo = setup();

        Ok(())
    }
}
