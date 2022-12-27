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
    T: SessionRepository + std::fmt::Debug,
{
    /// コンストラクタ
    pub fn new(session_repository: &Arc<T>) -> Self {
        Self {
            session_repository: Arc::clone(session_repository),
        }
    }

    #[tracing::instrument(skip(self), err, ret)]
    async fn create(&self) -> SessionResult<SessionStatus> {
        let session = SessionData::new();
        let session_id = self
            .session_repository
            .save(session)
            .await?
            .ok_or(SessionError::IntoSessionIdError)?;

        Ok(SessionStatus::Created(session_id))
    }
}

#[async_trait::async_trait]
impl<T> SessionService for SessionServiceImpl<T>
where
    T: SessionRepository + std::fmt::Debug + Send + Sync + Clone,
{
    /// Session取得 or 新規作成
    #[tracing::instrument(skip(self), err, ret)]
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
    #[tracing::instrument(skip(self), err)]
    async fn delete(&self, session_id: SessionId) -> SessionResult<()> {
        self.session_repository.delete(session_id).await
    }

    /// SessionItem取得
    #[tracing::instrument(skip(self), err, ret)]
    async fn find_item(
        &self,
        session_id: SessionId,
        key: &SessionItem,
    ) -> SessionResult<Option<SessionItem>> {
        let session = self
            .session_repository
            .find(session_id.clone())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        let item = session.item(key);
        Ok(item)
    }

    /// SessionItem保存
    #[tracing::instrument(skip(self), err)]
    async fn insert_item(&self, session_id: SessionId, item: SessionItem) -> SessionResult<()> {
        let mut session = self
            .session_repository
            .find(session_id.clone())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        session.insert_item(item)?;
        self.session_repository.save(session).await?;
        Ok(())
    }

    /// SessionItem削除
    #[tracing::instrument(skip(self), err)]
    async fn remove_item(&self, session_id: SessionId, key: &SessionItem) -> SessionResult<()> {
        let mut session = self
            .session_repository
            .find(session_id.clone())
            .await?
            .ok_or(SessionError::SessionNotFound(session_id))?;

        session.remove_item(key);
        self.session_repository.save(session).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::anyhow;
    use async_session::MemoryStore;

    use crate::session::{SessionRepositoryImpl, SessionServiceImpl};
    use presentation::{
        auth::AuthType,
        session::{SessionError, SessionId, SessionItem, SessionService, SessionStatus},
    };

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

        let SessionStatus::Created(session_id) = session_status else {
            return Err(anyhow!("session is not created"))
        };

        let SessionStatus::Found(saved_id) = session_service.find_or_create(Some(session_id.clone())).await? else {
            return Err(anyhow!("session is not saved"));
        };

        assert!(session_id == saved_id);

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
    async fn save_item_success() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let item = SessionItem::AuthType(AuthType::Singin);
        let key = SessionItem::AuthType(AuthType::Singin);

        service.insert_item(session_id.clone(), item).await?;

        assert!(service.find_item(session_id, &key).await?.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn save_item_to_noexist_session_return_err() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let item = SessionItem::AuthType(AuthType::Singin);
        service.delete(session_id.clone()).await?;

        let Err(SessionError::SessionNotFound(_)) = service.insert_item(session_id, item).await else {
            return Err(anyhow!("unexpected save item result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn get_notexist_item_return_none() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let key = SessionItem::AuthType(AuthType::Singin);

        assert!(service.find_item(session_id, &key).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn get_item_from_noexist_session_return_err() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let key = SessionItem::AuthType(AuthType::Singin);
        service.delete(session_id.clone()).await?;

        let Err(SessionError::SessionNotFound(_)) = service.find_item(session_id, &key).await else {
            return Err(anyhow!("unexpected save item result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn remove_item_success() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let item = SessionItem::AuthType(AuthType::Singin);
        let key = SessionItem::AuthType(AuthType::Singin);

        service.insert_item(session_id.clone(), item).await?;
        assert!(service.find_item(session_id.clone(), &key).await?.is_some());

        service.remove_item(session_id.clone(), &key).await?;
        assert!(service.find_item(session_id, &key).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn remove_item_from_noexist_session_return_err() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let key = SessionItem::AuthType(AuthType::Singin);
        service.delete(session_id.clone()).await?;

        let Err(SessionError::SessionNotFound(_)) = service.remove_item(session_id, &key).await else {
            return Err(anyhow!("unexpected save item result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn remove_notexist_item_return_ok() -> anyhow::Result<()> {
        let service = setup();
        let session_id: SessionId = service.find_or_create(None).await?.into();
        let key = SessionItem::AuthType(AuthType::Singin);

        service.remove_item(session_id, &key).await?;

        Ok(())
    }
}
