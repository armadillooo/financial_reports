use super::user_id::UserId;
use super::user_name::UserName;

/// Userドメインモデル
pub struct User {
    /// Unique id
    id: UserId,
    /// Unique name
    name: UserName,
}

impl User {
    /// コンストラクタ
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn change_name(&mut self, name: UserName) {
        self.name = name;
    }
}
