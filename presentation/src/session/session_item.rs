use serde::{Deserialize, Serialize};

use crate::{
    auth::{AuthType, OICDData},
    user::LoginUserId,
};

/// Sessionに保存されるデータ
#[derive(Debug, Serialize, Deserialize)]
pub enum SessionItem {
    LoginUserId(LoginUserId),
    AuthInfo(OICDData),
    AuthType(AuthType),
}

impl SessionItem {
    /// アイテム取り出しのキーを取得する
    pub const fn key(&self) -> &'static str {
        match self {
            Self::LoginUserId(_) => "LoginUserId",
            Self::AuthInfo(_) => "AuthInfo",
            Self::AuthType(_) => "AuthType",
        }
    }
}
