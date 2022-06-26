use super::user_id::UserId;
use super::user_model::User;
use super::user_name::UserName;

/// User永続化インターフェース
pub trait UserRepository {
    fn find(&self, id: &UserId) -> anyhow::Result<User>;
    fn find_by_name(&self, name: &UserName) -> anyhow::Result<User>;
    fn save(&self, user: &User) -> anyhow::Result<()>;
    fn delete(&self, user: User) -> anyhow::Result<()>;
}
