use async_session::SessionStore;
use async_trait::async_trait;

use presentation::session::{SessionData, SessionError, SessionRepository, SessionResult};

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
    async fn delete(&self, session: SessionData) -> SessionResult<()> {
        let session = session.into();
        self.store
            .destroy_session(session)
            .await
            .map_err(|_| SessionError::Disconnect)
    }

    /// Session取得
    async fn find(&self, session_id: String) -> SessionResult<Option<SessionData>> {
        if let Some(session) = self
            .store
            .load_session(session_id)
            .await
            .map_err(|_| SessionError::Disconnect)?
        {
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> SessionResult<String> {
        self.store
            .store_session(session.into())
            .await
            .map_err(|_| SessionError::Disconnect)?
            .ok_or(SessionError::Disconnect)
    }
}

#[cfg(test)]
mod tests {
    use async_session::MemoryStore;

    use crate::session::SessionRepositoryImpl;
    use presentation::session::SessionRepository;

    #[test]
    fn tests() -> anyhow::Result<()> {
        let repo = SessionRepositoryImpl::new(MemoryStore::new());

        let _ = repo.find("asdfasdfa".to_string());

        Ok(())
    }
}
