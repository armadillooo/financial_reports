use crate::users::{CreateCommand, DeleteCommand, GetCommand, UserData};

/// Userアプリケーションサービスインターフェース
pub trait UserApplicationService {
    fn get(&self, command: GetCommand) -> anyhow::Result<UserData>;
    fn save(&self, command: CreateCommand) -> anyhow::Result<()>;
    fn delete(&self, command: DeleteCommand) -> anyhow::Result<()>;
}
