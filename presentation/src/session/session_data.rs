use std::time::Duration;

use async_session::Session;

const USER_ID_KEY: &str = "user id";

#[derive(Debug, Clone)]
pub struct SessionData(Session);

impl SessionData {
    /// コンストラクタ
    pub fn new(user_id: impl Into<String>) -> Self {
        let mut session = Session::new();
        session
            .insert(USER_ID_KEY, user_id.into())
            .expect("User id serialization was unsuccessfull");
        // default expiry = 1 hours
        let expiry = Duration::from_secs(60 * 60);
        session.expire_in(expiry);

        Self(session)
    }

    /// session id取得
    pub fn id(&self) -> &str {
        self.0.id()
    }

    /// Sessionの有効期限を現在時刻＋Durationに設定する
    pub fn set_expiry(&mut self, expiry: Duration) {
        self.0.expire_in(expiry);
    }

    /// 有効期限のDurationを取得
    pub fn expires_in(&self) -> Option<Duration> {
        self.0.expires_in()
    }

    /// user id取得
    pub fn user_id(&self) -> Option<String> {
        self.0.get(USER_ID_KEY)
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
    use std::time::Duration;

    use async_session::Session;

    use super::SessionData;

    #[test]
    fn create_session_with_expiry() {
        let user_id = "user";
        let session = SessionData::new(user_id);

        assert_eq!(session.user_id().unwrap(), user_id);
        assert!(session.expires_in().is_some());
    }

    #[test]
    fn update_expiry() {
        let session = Session::new();
        let mut session = SessionData::from(session);

        assert!(session.expires_in().is_none());

        session.set_expiry(Duration::from_secs(120));

        assert!(session.expires_in().is_some());
    }

    #[test]
    fn user_id_not_found_return_none() {
        let session = Session::new();
        let session = SessionData::from(session);

        assert!(session.user_id().is_none())
    }
}
