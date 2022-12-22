use std::clone::Clone;
use std::sync::Arc;

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
        let session_id = self.session_repository.save(session).await?;

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
            if let Some(session) = self.session_repository.find(session_id).await? {
                let status = SessionStatus::Found(session.into_session_id()?);

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
        self.session_repository.delete(session_id).await
    }

    async fn item(
        &self,
        session_id: SessionId,
        key: &SessionItem,
    ) -> SessionResult<SessionItem> {
        let session = self.session_repository.find(session_id).await?.ok_or(SessionError::Disconnect)?;

        let item = session.item(key).ok_or(SessionError::ItemNotFound)?;
        Ok(item)
    }

    async fn insert_item(
        &self,
        session_id: SessionId,
        item: SessionItem,
    ) -> SessionResult<()> {
        let session = self.session_repository.find(session_id).await?;

        Ok(())
    }

    async fn remove_item(&self, session_id: SessionId, Key: &SessionItem) -> SessionResult<()> {
        let session = self.session_repository.find(session_id).await?;

        Ok(())
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
