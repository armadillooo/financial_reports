use super::common::user_data::UserData;
use super::create::create_command::CreateCommand;
use super::delete::delete_command::DeleteCommand;
use super::get::get_command::GetCommand;
use domain::users::{User, UserId, UserName, UserRepository, UserService};

/// User application service
pub struct UserApplicationService<T>
where
    T: UserRepository,
{
    user_repository: T,
    user_service: UserService<T>,
}

impl<T> UserApplicationService<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: T, user_service: UserService<T>) -> Self {
        Self {
            user_repository,
            user_service,
        }
    }

    /// User取得
    pub fn get(&self, command: GetCommand) -> anyhow::Result<UserData> {
        let id = UserId::new(command.id);
        let user = self.user_repository.find(&id)?;

        Ok(user.into())
    }

    /// User新規作成
    pub fn sign_up(&self, command: CreateCommand) -> anyhow::Result<UserData> {
        let id = UserId::new(command.id);
        let name = UserName::new(command.name);
        let user = User::new(id, name);

        if self.user_service.exists(&user) {
            return Err(anyhow::format_err!("User already exists"));
        }

        self.user_repository.save(&user)?;
        Ok(UserData::from(user))
    }

    /// User削除
    pub fn sign_out(&self, command: DeleteCommand) -> anyhow::Result<()> {
        let id = UserId::new(command.id);
        let name = UserName::new(command.name);
        let user = User::new(id, name);

        if !self.user_service.exists(&user) {
            return Err(anyhow::format_err!("There is no user"));
        }

        self.user_repository.delete(user)?;
        Ok(())
    }
}
