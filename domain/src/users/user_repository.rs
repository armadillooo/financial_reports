use crate::users::{User, UserId, UserName};

/// User永続化インターフェース
#[async_trait::async_trait]
pub trait UserRepository {
    async fn find(&self, id: &UserId) -> anyhow::Result<Option<User>>;
    async fn find_by_name(&self, name: &UserName) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: User) -> anyhow::Result<()>;
    async fn delete(&self, user: User) -> anyhow::Result<()>;
}
