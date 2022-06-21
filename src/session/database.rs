//! Session DBへの接続を行う
use std::sync::Arc;
use std::{fmt::Error, ops::Deref};

use async_redis_session::RedisSessionStore;
use async_session::{Session, SessionStore};
use axum::headers::Cookie;
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

    /// Sessionを作成し、user idを登録する
    pub async fn start_session(&self, user_id: UserId) -> anyhow::Result<HeaderMap> {
        let mut session = Session::new();

        // Sessionにuser_idを登録
        session.insert(SESSION_USER_ID_KEY, user_id)?;

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

    /// Session・Cookieを削除する
    pub async fn delete_session(&self, session: Session) -> anyhow::Result<HeaderMap> {
        self.destroy_session(session).await?;

        // Cookieを空文字に設定する
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("{}=", SESSION_COOKIE_NAME))?,
        );

        Ok(headers)
    }

    /// CookieにセットされているSessionIDからセッションを取得する
    pub async fn find_session(&self, cookies: &Cookie) -> anyhow::Result<Session> {
        let cookie_value = cookies.get(SESSION_COOKIE_NAME).ok_or(Error)?;
        let session = self
            .load_session(cookie_value.to_string())
            .await?
            .ok_or(Error)?;

        Ok(session)
    }

    /// User IDを取得
    pub fn find_user_id(&self, session: &Session) -> Option<UserId> {
        session.get(SESSION_USER_ID_KEY)
    }
}

impl Deref for Store {
    type Target = Arc<RedisSessionStore>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
