use crate::users::UserData;

/// Userアプリケーションサービスインターフェース
#[async_trait::async_trait]
pub trait UserApplicationService {
    async fn get(&self, id: &str) -> anyhow::Result<Option<UserData>>;
    async fn save(&self, user: UserData) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
