use anyhow::anyhow;
use async_session::SessionStore;
use async_trait::async_trait;

use presentation::session::{SessionData, SessionRepository};

#[derive(Debug, Clone)]
pub struct SessionRepositoryImpl<T: SessionStore> {
    store: T,
}

impl<T: SessionStore> SessionRepositoryImpl<T> {
    /// コンストラクタ
    #[allow(dead_code)]
    pub fn new(store: T) -> Self {
        Self { store }
    }
}

#[async_trait]
impl<T: SessionStore> SessionRepository for SessionRepositoryImpl<T> {
    /// Session削除
    async fn delete(&self, session: SessionData) -> anyhow::Result<()> {
        let session = session.into();
        self.store.destroy_session(session).await
    }

    /// Session取得
    async fn find(&self, session_id: String) -> anyhow::Result<Option<SessionData>> {
        if let Some(session) = self.store.load_session(session_id).await? {
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    async fn save(&self, session: SessionData) -> anyhow::Result<String> {
        self.store
            .store_session(session.into())
            .await?
            .ok_or_else(|| anyhow!("Cookie value was not set"))
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
