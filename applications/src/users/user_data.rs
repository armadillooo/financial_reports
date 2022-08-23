use serde::{Deserialize, Serialize};

use domain::users::User;

/// User Data Transfer Object
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl UserData {
    /// コンストラクタ
    pub fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

impl From<User> for UserData {
    /// ドメインモデルからの変換
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            name: user.name().to_string(),
            email: user.email().to_string(),
        }
    }
}
