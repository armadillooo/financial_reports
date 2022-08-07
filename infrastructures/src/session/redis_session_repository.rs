use std::sync::Arc;

use async_redis_session::RedisSessionStore;
use async_session::SessionStore;
use async_trait::async_trait;

use super::SessionDataImpl;
use presentation::session::SessionRepository;

pub struct RedisSessionRepository {
    store: Arc<RedisSessionStore>,
}

impl RedisSessionRepository {
    /// コンストラクタ
    pub fn new(store: Arc<RedisSessionStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
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
            Err(std::fmt::Error.into())
        }
    }
}
