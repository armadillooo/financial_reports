//! Userドメインサービス
use std::sync::Arc;

use crate::user::{UserDomainError, UserDomainResult, UserId, UserRepository};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserDomainService<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
}

impl<T> UserDomainService<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: &Arc<T>) -> Self {
        Self {
            user_repository: Arc::clone(user_repository),
        }
    }

    pub async fn exists(&self, user_id: &UserId) -> UserDomainResult<()> {
        if let Some(_user) = self.user_repository.find(user_id).await? {
            Ok(())
        } else {
            Err(UserDomainError::UserNotFound(user_id.clone()))
        }
    }
}
