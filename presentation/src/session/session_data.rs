use std::time::Duration;

pub trait SessionData {
    // Sessionから取り出せる値
    type SessionUserId;
    type SessionOICDInfo;

    fn id(&self) -> &str;
    fn user_id(&self) -> Option<Self::SessionUserId>;
    fn set_user_id(&mut self, user_id: Self::SessionUserId);
    fn expiry(&self) -> Option<Duration>;
    fn set_expiry(&mut self, expiry: Duration);
    fn oicd_info(&self) -> Option<Self::SessionOICDInfo>;
    fn set_oicd_info(&mut self, oicd_info: Self::SessionOICDInfo);
}
