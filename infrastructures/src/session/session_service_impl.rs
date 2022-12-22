use std::clone::Clone;
use std::sync::Arc;

use presentation::session::{
    SessionData, SessionError, SessionId, SessionRepository, SessionResult, SessionService,
    SessionStatus, SessionWithId,
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
}

#[async_trait::async_trait]
impl<T> SessionService for SessionServiceImpl<T>
where
    T: SessionRepository + Send + Sync + Clone,
{
    /// Session取得 or 新規作成
    async fn find_or_create(&self, session_id: SessionId) -> SessionResult<SessionStatus> {
        let session = if let Some(session) = self.session_repository.find(session_id).await? {
            SessionStatus::Found(session)
        } else {
            let session = self.create().await?;
            SessionStatus::Created(session)
        };

        Ok(session)
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> SessionResult<SessionId> {
        let cookie_value = self.session_repository.save(session).await?;
        Ok(SessionId::new(cookie_value))
    }

    /// Session削除
    async fn delete(&self, session: SessionData) -> SessionResult<()> {
        self.session_repository.delete(session).await
    }
}

#[cfg(test)]
mod tests {
    use std::matches;
    use std::sync::Arc;

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
        let session_id = session_service.create().await?.id;

        assert!(session_service.find_or_create(session_id).await?.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn save_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionData::new();
        let session_id = session_service.save(session).await?;

        assert!(session_service.find_or_create(session_id).await?.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionData::new();
        let session_id = session_service.save(session).await?;
        let session = session_service
            .find_or_create(session_id.clone())
            .await?
            .into();
        session_service.delete(session).await?;

        assert!(matches!(
            session_service.find_or_create(session_id).await?,
            SessionStatus::Created(_)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn delete_not_exist_session_return_ok() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionData::new();

        assert!(session_service.delete(session).await.is_ok());

        Ok(())
    }
}
