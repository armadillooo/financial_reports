//! Userドメインサービス
use super::user_model::User;
use super::user_repository::UserRepository;

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

    pub fn exists(&self, user: &User) -> bool {
        if let Ok(_) = self.user_repository.find(user.id()) {
            true
        } else {
            false
        }
    }
}
