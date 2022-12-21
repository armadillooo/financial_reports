use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::session::SessionItemKey;

pub const USER_ID: SessionItemKey<LoginedUserId> = SessionItemKey::new("user id");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginedUserId(String);

impl LoginedUserId {
    /// コンストラクタ
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Into<String> for LoginedUserId {
    fn into(self) -> String {
        self.0
    }
}

impl Deref for LoginedUserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
