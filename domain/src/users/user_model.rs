use crate::users::{UserEmail, UserId, UserName};

/// Userドメインモデル
#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
}

impl User {
    /// コンストラクタ
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> Self {
        Self { id, name, email }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &UserEmail {
        &self.email
    }
}
