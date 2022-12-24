use serde::{Deserialize, Serialize};

use domain::user::{User, UserEmail, UserId, UserName};

/// User Data Transfer Object
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
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

impl Into<User> for UserData {
    fn into(self) -> User {
        let id = UserId::new(self.id);
        let name = UserName::new(self.name);
        let email = UserEmail::new(self.email);

        User::new(id, name, email)
    }
}
