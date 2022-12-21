use crate::users::{UserData, UserApplicationResult};

/// Userアプリケーションサービスインターフェース
#[async_trait::async_trait]
pub trait UserService {
    async fn get(&self, id: &str) -> UserApplicationResult<Option<UserData>>;
    async fn save(&self, user: UserData) -> UserApplicationResult<()>;
    async fn delete(&self, id: &str) -> UserApplicationResult<()>;
}
