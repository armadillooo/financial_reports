//! Userドメインサービス
use std::sync::Arc;

use crate::users::{User, UserRepository};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserService<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
}

impl<T> UserService<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: &Arc<T>) -> Self {
        Self {
            user_repository: Arc::clone(user_repository),
        }
    }

    pub async fn exists(&self, user: &User) -> bool {
        if let Ok(Some(_user)) = self.user_repository.find(user.id()).await {
            true
        } else {
            false
        }
    }
}
