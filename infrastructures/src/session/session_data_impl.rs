use std::time::Duration;

use async_session::Session;

use crate::auth::OICDData;
use presentation::session::SessionData;

const USER_ID_KEY: &str = "user_id";

pub struct SessionDataImpl(Session);

impl SessionDataImpl {
    /// コンストラクタ
    pub fn new() -> Self {
        let mut session = Session::new();
        // default expiry = 1 hours
        let expiry = Duration::from_secs(60 * 60);
        session.expire_in(expiry);

        Self(session)
    }
}

impl SessionData for SessionDataImpl {
    type SessionUserId = String;
    type SessionOICDInfo = OICDData;

    /// Session Id取得
    fn session_id(&self) -> &str {
        self.0.id()
    }

    /// Session User Id取得
    fn user_id(&self) -> Option<Self::SessionUserId> {
        self.0.get(USER_ID_KEY)
    }

    /// Session User Id設定
    fn set_user_id(&mut self, user_id: Self::SessionUserId) {
        let _ = self.0.insert(USER_ID_KEY, user_id);
    }

    /// Session 有効期限取得
    fn expiry(&self) -> Option<Duration> {
        self.0.expires_in()
    }

    /// Session 有効期限設定
    fn set_expiry(&mut self, expiry: Duration) {
        self.0.expire_in(expiry)
    }

    /// OICD情報取得
    fn oicd_info(&self) -> Option<Self::SessionOICDInfo> {
        unimplemented!()
    }

    /// OICD情報設定
    fn set_oicd_info(&mut self, oicd_info: Self::SessionOICDInfo) {
        unimplemented!()
    }
}

impl Into<Session> for SessionDataImpl {
    fn into(self) -> Session {
        self.0
    }
}

impl From<Session> for SessionDataImpl {
    fn from(session: Session) -> Self {
        Self(session)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use async_session::Session;
    use presentation::session::SessionData;

    use crate::session::SessionDataImpl;

    #[test]
    fn create_session_with_expiry() {
        let user_id = "user".to_string();
        let mut session = SessionDataImpl::new();
        session.set_user_id(user_id.clone());

        assert_eq!(session.user_id().unwrap(), user_id);
        assert!(session.expiry().is_some());
    }

    #[test]
    fn update_expiry() {
        let session = Session::new();
        let mut session = SessionDataImpl::from(session);

        assert!(session.expiry().is_none());

        session.set_expiry(Duration::from_secs(120));

        assert!(session.expiry().is_some());
    }

    #[test]
    fn user_id_not_found_return_none() {
        let session = Session::new();
        let session = SessionDataImpl::from(session);

        assert!(session.user_id().is_none())
    }
}
