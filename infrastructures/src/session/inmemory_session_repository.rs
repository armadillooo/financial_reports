use anyhow::anyhow;
use async_session::{MemoryStore, SessionStore};
use async_trait::async_trait;

use presentation::session::SessionRepository;

use super::SessionDataImpl;

pub struct InMemorySessionRepository {
    store: MemoryStore,
}

impl InMemorySessionRepository {
    /// コンストラクタ
    #[allow(dead_code)]
    pub fn new(store: MemoryStore) -> Self {
        Self { store }
    }
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    type Data = SessionDataImpl;

    /// Session削除
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()> {
        let session = session.into();
        self.store.destroy_session(session).await
    }

    /// Session取得
    async fn find(&self, session_id: &str) -> anyhow::Result<Option<Self::Data>> {
        if let Some(session) = self.store.load_session(session_id.to_string()).await? {
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    ///
    /// 保存に成功した場合Session idを返す
    async fn save(&self, session: Self::Data) -> anyhow::Result<String> {
        if let Some(session_id) = self.store.store_session(session.into()).await? {
            Ok(session_id)
        } else {
            Err(anyhow!("Failed to save session data"))
        }
    }
}
