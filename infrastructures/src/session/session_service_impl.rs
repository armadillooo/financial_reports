use std::clone::Clone;
use std::sync::Arc;

use anyhow::anyhow;

use presentation::session::{
    SessionData, SessionFromRequest, SessionMetadata, SessionRepository, SessionService,
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
    async fn find_or_create(&self, cookie_value: String) -> anyhow::Result<SessionFromRequest> {
        let session = if let Some(session) = self.session_repository.find(&cookie_value).await? {
            SessionFromRequest::Found(SessionMetadata {
                inner: session,
                cookie_value: cookie_value,
            })
        } else {
            let session = self.create().await?;
            SessionFromRequest::Created(session)
        };

        Ok(session)
    }

    /// Session作成
    async fn create(&self) -> anyhow::Result<SessionMetadata> {
        let session = SessionData::new();
        let new_cookie = self.save(session).await?;
        let session = self
            .session_repository
            .find(&new_cookie)
            .await?
            .ok_or_else(|| anyhow!("Created session was not saved"))?;

        let session = SessionMetadata {
            inner: session,
            cookie_value: new_cookie,
        };
        Ok(session)
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> anyhow::Result<String> {
        self.session_repository.save(session).await
    }

    /// Session削除
    async fn delete(&self, session: SessionData) -> anyhow::Result<()> {
        self.session_repository.delete(session).await
    }
}

#[cfg(test)]
mod tests {
    use std::matches;
    use std::sync::Arc;

    use anyhow::anyhow;
    use async_session::MemoryStore;

    use crate::session::{SessionRepositoryImpl, SessionServiceImpl};
    use presentation::session::{SessionData, SessionFromRequest, SessionService};

    fn setup() -> SessionServiceImpl<SessionRepositoryImpl<MemoryStore>> {
        let store = MemoryStore::new();
        let session_repository = Arc::new(SessionRepositoryImpl::new(store));
        let session_service = SessionServiceImpl::new(&session_repository);

        session_service
    }

    #[tokio::test]
    async fn create_new_session_saved() -> anyhow::Result<()> {
        let session_service = setup();
        let dummy_id = base64::encode("dummy");
        let created_session = if let SessionFromRequest::Created(session) =
            session_service.find_or_create(dummy_id).await?
        {
            session
        } else {
            return Err(anyhow!("Session already exist."));
        };
        let cookie_value = created_session.cookie_value;

        let saved_session = if let SessionFromRequest::Found(session) =
            session_service.find_or_create(cookie_value).await?
        {
            session
        } else {
            return Err(anyhow!("Session is not saved"));
        };

        assert_eq!(created_session.inner, saved_session.inner);

        Ok(())
    }

    #[tokio::test]
    async fn save_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionData::new();
        let cookie_value = session_service.save(session).await?;

        assert!(matches!(
            session_service.find_or_create(cookie_value).await?,
            SessionFromRequest::Found(_)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionData::new();
        let cookie_value = session_service.save(session).await?;
        let session = session_service.find_or_create(cookie_value.clone()).await?.into();
        session_service.delete(session).await?;

        assert!(matches!(
            session_service.find_or_create(cookie_value).await?,
            SessionFromRequest::Created(_)
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
