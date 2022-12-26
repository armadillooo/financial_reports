use std::time::Duration;

use async_session::chrono::{DateTime, Utc};
use async_session::Session;

use crate::session::{SessionError, SessionId, SessionItem, SessionResult};

#[derive(Debug, PartialEq, Default)]
pub struct SessionData(Session);

impl SessionData {
    pub fn new() -> Self {
        let mut session = Session::new();
        // Default expiry = 1.0h
        session.expire_in(Duration::from_secs(60 * 60));
        Self(session)
    }

    /// Sessionの変更状態を取得する
    pub fn is_changed(&self) -> bool {
        self.0.data_changed()
    }

    /// Session ID取得
    pub fn into_session_id(self) -> SessionResult<SessionId> {
        let id = self
            .0
            .into_cookie_value()
            .ok_or(SessionError::IntoSessionIdError)?;

        Ok(SessionId::new(id))
    }

    /// 値をSessionに追加する
    ///
    /// Sessionへの保存は行われない
    pub fn insert_item(&mut self, item: SessionItem) -> SessionResult<()> {
        self.0
            .insert(&item.key(), item)
            .map_err(|_| SessionError::SavingItemError)
    }

    /// 値の取得
    pub fn item(&self, item: &SessionItem) -> Option<SessionItem> {
        let item = self.0.get(&item.key());

        item
    }

    /// 値の削除
    pub fn remove_item(&mut self, item: &SessionItem) {
        self.0.remove(&item.key())
    }

    /// Session期限取得
    pub fn limit(&self) -> Option<&DateTime<Utc>> {
        self.0.expiry()
    }

    /// Session有効期間設定
    pub fn set_limit(&mut self, expiry: Duration) {
        self.0.expire_in(expiry)
    }
}

impl Into<Session> for SessionData {
    fn into(self) -> Session {
        self.0
    }
}

impl From<Session> for SessionData {
    fn from(session: Session) -> Self {
        Self(session)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use crate::{
        session::{SessionData, SessionItem},
        user::LoginUserId,
    };

    #[test]
    fn create_session_with_expiry() {
        let session = SessionData::new();

        assert!(session.limit().is_some());
    }

    #[test]
    fn item_insert_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));

        assert!(session.insert_item(item).is_ok());

        Ok(())
    }

    #[test]
    fn item_get_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));
        let same_item = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));

        session.insert_item(item)?;

        let saved = session.item(&same_item).unwrap();

        if let (SessionItem::LoginUserId(item), SessionItem::LoginUserId(saved)) =
            (same_item, saved)
        {
            assert!(item == saved);
        } else {
            return Err(anyhow!("failed to save item"));
        }

        Ok(())
    }

    #[test]
    fn item_not_found_return_none() {
        let session = SessionData::new();
        let item = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));

        assert!(session.item(&item).is_none())
    }

    #[test]
    fn item_remove_success() -> anyhow::Result<()> {
        let mut session = SessionData::new();
        let item = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));
        let key = SessionItem::LoginUserId(LoginUserId::new("key".to_string()));

        session.insert_item(item)?;

        assert!(session.item(&key).is_some());

        session.remove_item(&key);

        assert!(session.item(&key).is_none());

        Ok(())
    }
}
