use domain::users::User;

/// User Data Transfer Object
pub struct UserData {
    pub id: String,
    pub name: String,
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
