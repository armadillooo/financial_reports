use crate::users::{User, UserId, UserName};

/// User永続化インターフェース
pub trait UserRepository {
    fn find(&self, id: &UserId) -> anyhow::Result<Option<User>>;
    fn find_by_name(&self, name: &UserName) -> anyhow::Result<Option<User>>;
    fn save(&self, user: User) -> anyhow::Result<()>;
    fn delete(&self, user: User) -> anyhow::Result<()>;
}
