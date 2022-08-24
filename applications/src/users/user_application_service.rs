use crate::users::{CreateCommand, DeleteCommand, GetCommand, UserData};

/// Userアプリケーションサービスインターフェース
#[async_trait::async_trait]
pub trait UserApplicationService {
    async fn get(&self, command: GetCommand) -> anyhow::Result<UserData>;
    async fn save(&self, command: CreateCommand) -> anyhow::Result<()>;
    async fn delete(&self, command: DeleteCommand) -> anyhow::Result<()>;
}
