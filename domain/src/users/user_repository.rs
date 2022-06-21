use crate::users::User;
use super::user_id::UserId;
use super::user_name::UserName;

/// User永続化インターフェース
pub trait UserRepository {
    fn find(&self, id: UserId) -> Option<User>;
    fn find_by_name(&self, name: UserName) -> Option<User>;
    fn find_all(&self) -> Option<Vec<User>>;
    fn save(&self) -> anyhow::Result<()>;
    fn delete(&self) -> anyhow::Result<()>;
    fn update(&self) -> anyhow::Result<()>;
}