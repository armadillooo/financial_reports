use anyhow::anyhow;
use async_session::{MemoryStore, SessionStore};
use async_trait::async_trait;

use presentation::session::SessionRepository;

use super::SessionDataImpl;

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
    type Data = SessionDataImpl;

    /// Session削除
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()> {
        let session = session.into();
        self.store.destroy_session(session).await
    }

    /// Session取得
    async fn find(&self, cookie_value: &str) -> anyhow::Result<Option<Self::Data>> {
        if let Some(session) = self.store.load_session(cookie_value.to_string()).await? {
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    async fn save(&self, session: Self::Data) -> anyhow::Result<String> {
        self.store
            .store_session(session.into())
            .await?
            .ok_or_else(|| anyhow!(""))
    }
}
