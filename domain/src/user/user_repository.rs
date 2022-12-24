use crate::user::{User, UserId, UserName, UserDomainResult};

/// User永続化インターフェース
#[async_trait::async_trait]
pub trait UserRepository {
    async fn find(&self, id: &UserId) -> UserDomainResult<Option<User>>;
    async fn find_by_name(&self, name: &UserName) -> UserDomainResult<Option<User>>;
    async fn save(&self, user: User) -> UserDomainResult<()>;
    async fn delete(&self, user: User) -> UserDomainResult<()>;
}
