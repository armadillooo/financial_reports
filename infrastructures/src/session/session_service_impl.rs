use std::sync::Arc;

use anyhow::anyhow;

use crate::session::SessionDataImpl;
use presentation::session::{
    CreatedSession, SessionFromRequest, SessionRepository, SessionService,
};

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
    T: SessionRepository<Data = SessionDataImpl> + Send + Sync,
{
    type Data = SessionDataImpl;

    /// Session取得 or 新規作成
    async fn find_or_create(
        &self,
        cookie_value: &str,
    ) -> anyhow::Result<SessionFromRequest<Self::Data>> {
        let session = if let Some(session) = self.session_repository.find(cookie_value).await? {
            SessionFromRequest::Found(session)
        } else {
            let session = SessionDataImpl::new();
            let cookie = self.save(session).await?;
            let session = self
                .session_repository
                .find(&cookie)
                .await?
                .ok_or_else(|| anyhow!("Failed to save new session"))?;

            SessionFromRequest::Created(CreatedSession { session, cookie })
        };

        Ok(session)
    }

    /// Session保存
    async fn save(&self, session: Self::Data) -> anyhow::Result<String> {
        self.session_repository.save(session).await
    }

    /// Session削除
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()> {
        self.session_repository.delete(session).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::matches;

    use anyhow::anyhow;
    use async_session::MemoryStore;

    use crate::session::{
        session_repository_impl::SessionRepositoryImpl, SessionDataImpl, SessionServiceImpl,
    };
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
            session_service.find_or_create(&dummy_id).await?
        {
            session
        } else {
            return Err(anyhow!("Session already exist."));
        };
        let cookie_value = created_session.cookie;

        let saved_session = if let SessionFromRequest::Found(session) =
            session_service.find_or_create(&cookie_value).await?
        {
            session
        } else {
            return Err(anyhow!("Session is not saved"));
        };

        assert_eq!(created_session.session, saved_session);

        Ok(())
    }

    #[tokio::test]
    async fn save_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let mut session = SessionDataImpl::new();
        let user_id = "sample user".to_string();
        session.set_user_id(user_id.clone());
        let cookie_value = session_service.save(session).await?;
        let saved_session: SessionDataImpl = session_service.find_or_create(&cookie_value).await?.into();

        assert_eq!(
            user_id,
            saved_session.user_id().expect("User id was not saved")
        );

        Ok(())
    }

    #[tokio::test]
    async fn delete_session_success() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionDataImpl::new();
        let cookie_value = session_service.save(session).await?;
        let session = session_service.find_or_create(&cookie_value).await?.into();
        session_service.delete(session).await?;

        assert!(matches!(session_service.find_or_create(&cookie_value).await? , SessionFromRequest::Created(_)));
        
        Ok(())
    }

    #[tokio::test]
    async fn delete_not_exist_session_return_ok() -> anyhow::Result<()> {
        let session_service = setup();
        let session = SessionDataImpl::new();

        assert!(session_service.delete(session).await.is_ok());

        Ok(())
    }
}
