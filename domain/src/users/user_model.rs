use crate::{users::{UserEmail, UserId, UserName}, stock::StockId};

/// Userドメインモデル
#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    name: UserName,
    email: UserEmail,
    // お気に入り登録
    favorites: Vec<StockId>,
    // 保有資産
    portfolio: Vec<StockId>,
}

impl User {
    /// コンストラクタ
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> Self {
        Self { id, name, email, favorites: vec![] }
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
