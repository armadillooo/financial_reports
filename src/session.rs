pub mod database;
pub mod request;

/// Session IDを格納するCookieのKey
const SESSION_COOKIE_NAME: &str = "session";
/// SessionからUser idを取り出すためのKey
const SESSION_USER_ID_KEY: &str = "user_id";
