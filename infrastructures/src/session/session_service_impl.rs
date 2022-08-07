use std::sync::Arc;

use presentation::session::{SessionData, SessionRepository, SessionService};

use crate::session::SessionDataImpl;

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

    /// Session新規作成
    async fn start_session(&self, user_id: String) -> anyhow::Result<String> {
        let mut session = SessionDataImpl::new();
        session.set_user_id(user_id);

        let session_id = self.session_repository.save(session).await?;

        Ok(session_id)
    }

    /// Session削除
    async fn delete_session(&self, session: Self::Data) -> anyhow::Result<()> {
        self.session_repository.delete(session).await
    }

    /// Session取得
    async fn load_session(&self, session_id: &str) -> anyhow::Result<Option<Self::Data>> {
        self.session_repository.find(&session_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_session::MemoryStore;

    use crate::session::{
        inmemory_session_repository::InMemorySessionRepository, SessionDataImpl, SessionServiceImpl,
    };
    use presentation::session::{SessionData, SessionService};

    fn setup() -> SessionServiceImpl<InMemorySessionRepository> {
        let store = MemoryStore::new();
        let session_repository = Arc::new(InMemorySessionRepository::new(store));
        let session_service = SessionServiceImpl::new(&session_repository);

        session_service
    }

    #[tokio::test]
    async fn create_new_session_saved() -> anyhow::Result<()> {
        let session_service = setup();
        let user_id = "dummy".to_string();

        let session_id = session_service.start_session(user_id.clone()).await?;
        assert_eq!(
            session_service
                .load_session(&session_id)
                .await?
                .unwrap()
                .user_id(),
            Some(user_id)
        );

        Ok(())
    }

    #[tokio::test]
    async fn load_not_exist_session_return_none() -> anyhow::Result<()> {
        let session_service = setup();
        let session_id = "dummy";
        let session_id = base64::encode(session_id);

        assert!(session_service.load_session(&session_id).await?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn delete_not_exist_session_return_ok() -> anyhow::Result<()> {
        let session_service = setup();
        let user_id = "delete user".to_string();
        let mut session = SessionDataImpl::new();
        session.set_user_id(user_id);
        let session_id = session.session_id();

        assert!(session_service.load_session(session_id).await?.is_none());
        assert!(session_service.delete_session(session).await.is_ok());

        Ok(())
    }
}
