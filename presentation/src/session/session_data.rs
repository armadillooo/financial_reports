use std::time::Duration;

use anyhow::Context;
use async_session::Session;
use serde::Serialize;

use crate::session::{SessionFromRequest, SessionKey};

#[derive(Debug, PartialEq)]
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

    /// 値をSessionに追加する
    ///
    /// Sessionへの保存は行われない
    pub fn insert<T: Serialize>(&mut self, key: SessionKey<T>, value: T) -> anyhow::Result<()> {
        self.inner
            .insert(&key.value, value)
            .context("Serialization fault")
    }

    /// 値の取得
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: SessionKey<T>) -> Option<T> {
        self.inner.get(&key.value)
    }

    /// 値の削除
    fn remove<T>(&mut self, key: SessionKey<T>) {
        self.inner.remove(&key.value)
    }

    /// Session期限取得
    fn expiry(&self) -> Option<Duration> {
        self.inner.expires_in()
    }

    /// Session期限設定
    fn set_expiry(&mut self, expiry: Duration) {
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
            SessionFromRequest::Found(session) => session,
            SessionFromRequest::Created(info) => info.session,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_session::Session;

    use crate::session::{SessionData, SessionKey};

    #[test]
    fn create_session_with_expiry() {
        let session = SessionData::new();

        assert!(session.expiry().is_some());
    }

    #[test]
    fn update_expiry() {
        let session = Session::new();
        let mut session = SessionData::from(session);

        assert!(session.expiry().is_none());

        session.set_expiry(Duration::from_secs(120));

        assert!(session.expiry().is_some());
    }

    #[test]
    fn item_insert_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = vec![1, 2, 3];
        let key = SessionKey::new("key".to_string());
        session.insert(key, item)?;

        Ok(())
    }

    #[test]
    fn item_get_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = "sample data".to_string();
        let key = SessionKey::new("key".to_string());
        session.insert(key.clone(), item.clone())?;

        assert_eq!(item, session.get(key).expect("Item was not saved"));

        Ok(())
    }

    #[test]
    fn item_not_found_return_none() {
        let session = SessionData::new();
        let key: SessionKey<String> = SessionKey::new("key".to_string());

        assert!(session.get(key).is_none())
    }

    #[test]
    fn item_remove_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let key: SessionKey<String> = SessionKey::new("key".to_string());
        let item = "item".to_string();
        session.insert(key.clone(), item)?;

        assert!(session.get(key.clone()).is_some());

        session.remove(key.clone());

        assert!(session.get(key).is_none());

        Ok(())
    }
}
