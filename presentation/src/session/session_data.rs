use std::time::Duration;

use anyhow::Context;
use async_session::chrono::{DateTime, Utc};
use async_session::Session;
use serde::Serialize;

use crate::session::{ItemKey, SessionFromRequest};

use super::SessionId;

#[derive(Debug, PartialEq, Clone)]
pub struct SessionData {
    inner: Session,
}

impl SessionData {
    pub fn new() -> Self {
        let mut session = Session::new();
        // Default expiry = 1.0h
        session.expire_in(Duration::from_secs(60 * 60));
        Self { inner: session }
    }

    /// Session Idをリセットする
    pub fn reset_id(&mut self, id: SessionId) {
        self.inner.set_cookie_value(id.into());
    }

    /// Sessionの変更状態を取得する
    pub fn is_changed(&self) -> bool {
        self.inner.data_changed()
    }

    /// 値をSessionに追加する
    ///
    /// Sessionへの保存は行われない
    pub fn insert_item<T: Serialize>(&mut self, key: &ItemKey<T>, item: T) -> anyhow::Result<()> {
        self.inner
            .insert(&key.value, item)
            .context("Serialization fault")
    }

    /// 値の取得
    pub fn item<T: serde::de::DeserializeOwned>(&self, key: &ItemKey<T>) -> Option<T> {
        self.inner.get(&key.value)
    }

    /// 値の削除
    pub fn remove_item<T>(&mut self, key: &ItemKey<T>) {
        self.inner.remove(&key.value)
    }

    /// Session期限取得
    pub fn limit(&self) -> Option<&DateTime<Utc>> {
        self.inner.expiry()
    }

    /// Session有効期間設定
    pub fn set_limit(&mut self, expiry: Duration) {
        self.inner.expire_in(expiry)
    }
}

impl Into<Session> for SessionData {
    fn into(self) -> Session {
        self.inner
    }
}

impl From<Session> for SessionData {
    fn from(session: Session) -> Self {
        Self { inner: session }
    }
}

impl From<SessionFromRequest> for SessionData {
    fn from(from_request: SessionFromRequest) -> Self {
        match from_request {
            SessionFromRequest::Found(session) => session.inner,
            SessionFromRequest::Refreshed(session) => session.inner,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::session::{ItemKey, SessionData};

    #[test]
    fn create_session_with_expiry() {
        let session = SessionData::new();

        assert!(session.limit().is_some());
    }

    #[test]
    fn item_insert_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = vec![1, 2, 3];
        let key = ItemKey::new("key".to_string());
        session.insert_item(&key, item)?;

        Ok(())
    }

    #[test]
    fn item_get_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = "sample data".to_string();
        let key = ItemKey::new("key".to_string());
        session.insert_item(&key, item.clone())?;

        assert_eq!(item, session.item(&key).expect("Item was not saved"));

        Ok(())
    }

    #[test]
    fn item_not_found_return_none() {
        let session = SessionData::new();
        let key: ItemKey<String> = ItemKey::new("key".to_string());

        assert!(session.item(&key).is_none())
    }

    #[test]
    fn item_remove_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let key: ItemKey<String> = ItemKey::new("key".to_string());
        let item = "item".to_string();
        session.insert_item(&key, item)?;

        assert!(session.item(&key).is_some());

        session.remove_item(&key);

        assert!(session.item(&key).is_none());

        Ok(())
    }
}
