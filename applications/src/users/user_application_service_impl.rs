use std::sync::Arc;

use crate::users::{CreateCommand, DeleteCommand, GetCommand, UserApplicationService, UserData};
use domain::users::{User, UserId, UserName, UserRepository, UserService};

/// User application service
#[derive(Debug, Clone)]
pub struct UserApplicationServiceImpl<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
    user_service: UserService<T>,
}

impl<T> UserApplicationServiceImpl<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: &Arc<T>) -> Self {
        Self {
            user_repository: Arc::clone(user_repository),
            user_service: UserService::new(user_repository),
        }
    }
}

impl<T> UserApplicationService for UserApplicationServiceImpl<T>
where
    T: UserRepository,
{
    /// User取得
    fn get(&self, command: GetCommand) -> anyhow::Result<UserData> {
        let id = UserId::new(command.id);
        let user = self
            .user_repository
            .find(&id)?
            .ok_or(anyhow::format_err!("User not found"))?;

        Ok(user.into())
    }

    /// User新規作成
    fn save(&self, command: CreateCommand) -> anyhow::Result<()> {
        let id = UserId::new(command.id);
        let name = UserName::new(command.name);
        let user = User::new(id, name);

        if self.user_service.exists(&user) {
            return Err(anyhow::format_err!("User already exists"));
        }

        self.user_repository.save(user)?;
        Ok(())
    }

    /// User削除
    fn delete(&self, command: DeleteCommand) -> anyhow::Result<()> {
        let id = UserId::new(command.id);
        if let Some(user) = self.user_repository.find(&id)? {
            self.user_repository.delete(user)?;
        } else {
            // ユーザーが存在しなかった場合は削除成功扱い
            return Ok(());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod user_usecase_tests {
        use std::sync::Arc;

        use crate::users::inmemory_user_repository::InMemoryUserRepository;
        use crate::users::{
            CreateCommand, DeleteCommand, GetCommand, UserApplicationService,
            UserApplicationServiceImpl, UserData,
        };

        // テストに必要なオブジェクトの初期化
        fn setup() -> UserApplicationServiceImpl<InMemoryUserRepository> {
            let user_repository = Arc::new(InMemoryUserRepository::new());
            let user_application = UserApplicationServiceImpl::new(&user_repository);

            user_application
        }

        #[test]
        fn create_user_saved() {
            let app_service = setup();
            let id = "1";
            let name = "hoge";
            let create_command = CreateCommand::new(id, name);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name);

            assert!(app_service.save(create_command).is_ok());

            let get_user = app_service.get(get_command).unwrap();
            assert_eq!(get_user, created_user);
        }

        #[test]
        fn create_same_user_not_saved() {
            let app_service = setup();

            let id = "1";
            let name1 = "hoge";
            let create_command = CreateCommand::new(id, name1);
            let name2 = "sample name";
            let create_same_user_command = CreateCommand::new(id, name2);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name1);

            assert!(app_service.save(create_command).is_ok());

            assert!(app_service.save(create_same_user_command).is_err());

            let get_user = app_service.get(get_command).unwrap();
            assert_eq!(get_user, created_user);
        }

        #[test]
        fn get_not_exist_user_return_error() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);

            assert!(app_service.get(get_command).is_err())
        }

        #[test]
        fn delete_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let name = "delete user";
            let created_user = UserData::new(id, name);
            let create_command = CreateCommand::new(id, name);
            let delete_command = DeleteCommand::new(id);
            let get_command = GetCommand::new(id);

            assert!(app_service.save(create_command).is_ok());

            assert_eq!(app_service.get(get_command).unwrap(), created_user);

            assert!(app_service.delete(delete_command).is_ok());

            let get_command = GetCommand::new(id);
            assert!(app_service.get(get_command).is_err());
        }

        #[test]
        fn delete_not_exist_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);
            let delete_command = DeleteCommand::new(id);

            assert!(app_service.get(get_command).is_err());

            assert!(app_service.delete(delete_command).is_ok());
        }
    }
}
