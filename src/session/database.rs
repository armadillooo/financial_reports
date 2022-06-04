//! Session DBへの接続を行う
use std::sync::Arc;
use std::ops::Deref;

use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use axum::http::{self, HeaderMap, HeaderValue};

use super::{request::UserId, SESSION_COOKIE_NAME, SESSION_USER_ID_KEY};

/// Session DB接続インターフェース
///
/// 非同期タスク間で共有するためArc<>に内包させる
#[derive(Clone)]
pub struct Store(pub(crate) Arc<RedisSessionStore>);

impl Store {
    /// Session DBとのネクション確立
    pub fn new() -> Store {
        let store_url = dotenvy::var("STORE_URL").unwrap();

        let store = RedisSessionStore::new(store_url).unwrap();

        Store(Arc::new(store))
    }

    /// Sessionを新規作成する
    pub async fn create_session(&self, user_id: UserId) -> anyhow::Result<HeaderMap> {
        let mut session = Session::new();

        // Sessionにuser_idを登録
        session.insert(SESSION_USER_ID_KEY, user_id.0)?;

        // SessionをStoreに保存する
        let cookie = (*self.0).store_session(session).await?.unwrap();

        // HttpレスポンスのヘッダーにSet-Cookieフィールドを設定
        // クライアントPCのCookieにSession IDを保存させる
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("{}={}", SESSION_COOKIE_NAME, cookie)).unwrap(),
        );

        Ok(headers)
    }

    /// Sessionを削除する
    pub async fn destroy_session(&self, session: Session) -> anyhow::Result<()> {
        (*self.0).destroy_session(session).await
    }

    /// SessionのCookieを更新する
    pub async fn regrate_session(&self, cookie: &str) -> anyhow::Result<()> {}
}

impl Deref for Store {
    type Target = Arc<RedisSessionStore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
