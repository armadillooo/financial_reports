use crate::users::{UserId, UserName};

/// Userドメインモデル
#[derive(Clone, Debug)]
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
