use applications::session::{SessionData, SessionRepository};

pub struct SessionService<T>
where
    T: SessionRepository,
{
    session_repository: T,
}

impl<T> SessionService<T>
where
    T: SessionRepository,
{
    /// コンストラクタ
    pub fn new(session_repository: T) -> Self {
        Self { session_repository }
    }

    /// Session新規作成
    pub async fn start_session(&self, user_id: &str) -> anyhow::Result<String> {
        let session = SessionData::new(user_id);

        let session_id = self.session_repository.save(session).await?;

        Ok(session_id)
    }

    /// Session削除
    pub async fn delete_session(&self, session: SessionData) -> anyhow::Result<()> {
        self.session_repository.delete(session).await
    }

    /// Session取得
    pub async fn load_session(&self, session_id: &str) -> anyhow::Result<Option<SessionData>> {
        self.session_repository.find(session_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use applications::session::{InMemorySessionRepository, SessionData};
    use async_session::MemoryStore;

    use super::SessionService;

    fn setup() -> SessionService<InMemorySessionRepository> {
        let store = Arc::new(MemoryStore::new());
        let session_repository = InMemorySessionRepository::new(store);
        let session_service = SessionService::new(session_repository);

        session_service
    }

    #[tokio::test]
    async fn create_new_session_saved() -> anyhow::Result<()> {
        let session_service = setup();
        let user_id = "dummy".to_string();

        let session_id = session_service.start_session(&user_id).await?;
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
        let user_id = "delete user";
        let session = SessionData::new(user_id);
        let session_id = session.id();

        assert!(session_service.load_session(session_id).await?.is_none()); 
        assert!(session_service.delete_session(session).await.is_ok());

        Ok(())
    }
}
