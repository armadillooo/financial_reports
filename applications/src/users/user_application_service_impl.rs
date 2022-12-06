use std::sync::Arc;

use crate::users::{CreateCommand, DeleteCommand, GetCommand, UserApplicationService, UserData};
use domain::users::{UserId, UserRepository, UserService};

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

#[async_trait::async_trait]
impl<T> UserApplicationService for UserApplicationServiceImpl<T>
where
    T: UserRepository + Send + Sync,
{
    /// User取得
    async fn get(&self, command: GetCommand) -> anyhow::Result<Option<UserData>> {
        let id = UserId::new(command.id);

        self.user_repository
            .find(&id)
            .await
            .map(|found| found.map(|found| UserData::from(found)))
    }

    /// User新規作成
    async fn save(&self, command: CreateCommand) -> anyhow::Result<()> {
        let user = command.user.into();

        if self.user_service.exists(&user).await {
            return Err(anyhow::format_err!("User already exists"));
        }

        self.user_repository.save(user).await?;
        Ok(())
    }

    /// User削除
    async fn delete(&self, command: DeleteCommand) -> anyhow::Result<()> {
        let id = UserId::new(command.id);
        if let Some(user) = self.user_repository.find(&id).await? {
            self.user_repository.delete(user).await?;
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

        use crate::users::inmemory_user_repository_impl::InMemoryUserRepositoryImpl;
        use crate::users::{
            CreateCommand, DeleteCommand, GetCommand, UserApplicationService,
            UserApplicationServiceImpl, UserData,
        };

        // テストに必要なオブジェクトの初期化
        fn setup() -> UserApplicationServiceImpl<InMemoryUserRepositoryImpl> {
            let user_repository = Arc::new(InMemoryUserRepositoryImpl::new());
            let user_application = UserApplicationServiceImpl::new(&user_repository);

            user_application
        }

        #[tokio::test]
        async fn create_user_saved() {
            let app_service = setup();
            let id = "1";
            let name = "hoge";
            let email = "mail";
            let create_user = UserData::new(id, name, email);
            let create_command = CreateCommand::new(create_user);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name, email);

            assert!(app_service.save(create_command).await.is_ok());

            let get_user = app_service.get(get_command).await.unwrap();
            assert_eq!(get_user.unwrap(), created_user);
        }

        #[tokio::test]
        async fn create_same_user_not_saved() {
            let app_service = setup();

            let id = "1";
            let name1 = "hoge";
            let email1 = "fuga";
            let user1 = UserData::new(id, name1, email1);
            let create_command = CreateCommand::new(user1);
            let name2 = "sample name";
            let email2 = "abc";
            let user2 = UserData::new(id, name2, email2);
            let create_same_user_command = CreateCommand::new(user2);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name1, email1);

            assert!(app_service.save(create_command).await.is_ok());

            assert!(app_service.save(create_same_user_command).await.is_err());

            let get_user = app_service.get(get_command).await.unwrap();
            assert_eq!(get_user.unwrap(), created_user);
        }

        #[tokio::test]
        async fn get_not_exist_user_return_error() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);

            assert!(app_service.get(get_command).await.is_err())
        }

        #[tokio::test]
        async fn delete_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let name = "delete user";
            let email = "hoge";
            let created_user = UserData::new(id, name, email);
            let create_command = CreateCommand::new(created_user.clone());
            let delete_command = DeleteCommand::new(id);
            let get_command = GetCommand::new(id);

            assert!(app_service.save(create_command).await.is_ok());

            assert_eq!(
                app_service.get(get_command).await.unwrap().unwrap(),
                created_user
            );

            assert!(app_service.delete(delete_command).await.is_ok());

            let get_command = GetCommand::new(id);
            assert!(app_service.get(get_command).await.is_err());
        }

        #[tokio::test]
        async fn delete_not_exist_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);
            let delete_command = DeleteCommand::new(id);

            assert!(app_service.get(get_command).await.is_err());

            assert!(app_service.delete(delete_command).await.is_ok());
        }
    }
}
