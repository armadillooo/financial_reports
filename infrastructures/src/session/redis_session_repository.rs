use std::sync::Arc;

use async_redis_session::RedisSessionStore;
use async_session::SessionStore;
use async_trait::async_trait;

use presentation::session::{SessionData, SessionRepository};

pub struct RedisSessionRepository {
    store: Arc<RedisSessionStore>,
}

impl RedisSessionRepository {
    pub fn new(store: Arc<RedisSessionStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
    /// Session削除
    async fn delete(&self, session: SessionData) -> anyhow::Result<()> {
        let session = session.into();
        self.store.destroy_session(session).await
    }

    /// Session取得
    async fn find(&self, session_id: &str) -> anyhow::Result<Option<SessionData>> {
        if let Some(session) = self.store.load_session(session_id.to_string()).await? {
            Ok(Some(session.into()))
        } else {
            Ok(None)
        }
    }

    /// Session保存
    ///
    /// 保存に成功した場合Session idを返す
    async fn save(&self, session: SessionData) -> anyhow::Result<String> {
        if let Some(session_id) = self.store.store_session(session.into()).await? {
            Ok(session_id)
        } else {
            Err(std::fmt::Error.into())
        }
    }
}
