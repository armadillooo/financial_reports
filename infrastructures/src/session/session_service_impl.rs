use std::sync::Arc;

use tracing::error;

use presentation::session::{
    SessionData, SessionError, SessionId, SessionItem, SessionRepository, SessionResult,
    SessionService, SessionStatus,
};

#[derive(Debug, Clone)]
pub struct SessionServiceImpl<T>
where
    T: SessionRepository,
{
    session_repository: Arc<T>,
}

impl<T> SessionServiceImpl<T>
where
    T: SessionRepository,
{
    /// コンストラクタ
    pub fn new(session_repository: &Arc<T>) -> Self {
        Self {
            session_repository: Arc::clone(session_repository),
        }
    }

    async fn create(&self) -> SessionResult<SessionStatus> {
        let session = SessionData::new();
        let session_id = self.session_repository.save(session).await.map_err(|e| {
            error!("{}", e);
            e
        })?;

        Ok(SessionStatus::Created(session_id))
    }
}

#[async_trait::async_trait]
impl<T> SessionService for SessionServiceImpl<T>
where
    T: SessionRepository + Send + Sync + Clone,
{
    /// Session取得 or 新規作成
    async fn find_or_create(&self, session_id: Option<SessionId>) -> SessionResult<SessionStatus> {
        let status = if let Some(session_id) = session_id {
            if let Some(session) = self
                .session_repository
                .find(session_id)
                .await
                .map_err(|e| {
                    error!("{}", e);
                    e
                })?
            {
                let status = SessionStatus::Found(session.into_session_id().map_err(|e| {
                    error!("{}", e);
                    e
                })?);

                status
            } else {
                self.create().await?
            }
        } else {
            self.create().await?
        };

        Ok(status)
    }

    /// Session削除
    async fn delete(&self, session_id: SessionId) -> SessionResult<()> {
        self.session_repository
            .delete(session_id)
            .await
            .map_err(|e| {
                error!("{}", e);
                e
            })
    }

    async fn item(&self, session_id: SessionId, key: &SessionItem) -> SessionResult<SessionItem> {
        let session = self
            .session_repository
            .find(session_id)
            .await?
            .ok_or(SessionError::SessionNotFound)
            .map_err(|e| {
                error!("{}", e);
                e
            })?;

        let item = session
            .item(key)
            .ok_or(SessionError::ItemNotFound)
            .map_err(|e| {
                error!("{}", e);
                e
            })?;
        Ok(item)
    }

    async fn insert_item(&self, session_id: SessionId, item: SessionItem) -> SessionResult<()> {
        let mut session = self
            .session_repository
            .find(session_id)
            .await?
            .ok_or(SessionError::SessionNotFound)
            .map_err(|e| {
                error!("{}", e);
                e
            })?;

        session.insert_item(item)
    }

    async fn remove_item(&self, session_id: SessionId, key: &SessionItem) -> SessionResult<()> {
        let mut session = self
            .session_repository
            .find(session_id)
            .await
            .map_err(|e| {
                error!("{}", e);
                e
            })?
            .ok_or(SessionError::SessionNotFound)
            .map_err(|e| {
                error!("{}", e);
                e
            })?;

        session.remove_item(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::anyhow;
    use async_session::MemoryStore;

    use crate::session::{SessionRepositoryImpl, SessionServiceImpl};
    use presentation::session::{SessionData, SessionService, SessionStatus};

    fn setup() -> SessionServiceImpl<SessionRepositoryImpl<MemoryStore>> {
        let store = MemoryStore::new();
        let session_repository = Arc::new(SessionRepositoryImpl::new(store));
        let session_service = SessionServiceImpl::new(&session_repository);

        session_service
    }

    #[tokio::test]
    async fn create_new_session_saved() -> anyhow::Result<()> {
        let session_service = setup();
        let session_status = session_service.find_or_create(None).await?;

        let SessionStatus::Found(_) = session_status else {
            return Err(anyhow!("session not found"))
        };

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session_id = session_service.find_or_create(None).await?.into();

        // セッションが保存されていることの確認
        let SessionStatus::Found(session_id) = session_service.find_or_create(Some(session_id)).await? else {
            return Err(anyhow!("session is not saved"));
        };

        session_service.delete(session_id.clone()).await?;
        // セッションが削除されていることの確認
        let SessionStatus::Created(_) = session_service.find_or_create(Some(session_id)).await? else {
            return Err(anyhow!("session is not removed"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn delete_not_exist_session_return_ok() -> anyhow::Result<()> {
        let session_service = setup();
        let session_id = SessionData::new().into_session_id()?;

        assert!(session_service.delete(session_id).await.is_ok());

        Ok(())
    }
}
