//! Userドメインサービス
use super::user_repository::{self, UserRepository};
use super::User;

pub struct UserService<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> UserService<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub fn exists(&self, user: User) -> Option<User> {
        self.user_repository.find(user.id)
    }
}
