use domain::users::User;

/// User Data Transfer Object
#[derive(Debug, PartialEq, Eq)]
pub struct UserData {
    pub id: String,
    pub name: String,
}

impl UserData {
    /// コンストラクタ
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl From<User> for UserData {
    /// ドメインモデルからの変換
    fn from(user: User) -> Self {
        Self {
            id: user.id().value().to_string(),
            name: user.name().value().to_string(),
        }
    }
}
