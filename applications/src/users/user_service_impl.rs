use std::sync::Arc;

use crate::users::{UserApplicationResult, UserData, UserService};
use domain::users::{UserDomainService, UserId, UserRepository};

/// User application service
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserServiceImpl<T>
where
    T: UserRepository,
{
    user_repository: Arc<T>,
    user_service: UserDomainService<T>,
}

impl<T> UserServiceImpl<T>
where
    T: UserRepository,
{
    /// コンストラクタ
    pub fn new(user_repository: &Arc<T>) -> Self {
        Self {
            user_repository: Arc::clone(user_repository),
            user_service: UserDomainService::new(user_repository),
        }
    }
}

#[async_trait::async_trait]
impl<T> UserService for UserServiceImpl<T>
where
    T: UserRepository + Send + Sync,
{
    /// User取得
    async fn get(&self, id: &str) -> UserApplicationResult<Option<UserData>> {
        let user = self
            .user_repository
            .find(&UserId::new(id.to_string()))
            .await?
            .map(|found| UserData::from(found));

        Ok(user)
    }

    /// User新規作成
    async fn save(&self, user: UserData) -> UserApplicationResult<()> {
        let user_id = UserId::new(user.id.clone());

        self.user_service.exists(&user_id).await?;

        self.user_repository.save(user.into()).await?;
        Ok(())
    }

    /// User削除
    async fn delete(&self, id: &str) -> UserApplicationResult<()> {
        let id = UserId::new(id.to_string());
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
        use crate::users::{UserData, UserService, UserServiceImpl};

        // テストに必要なオブジェクトの初期化
        fn setup() -> UserServiceImpl<InMemoryUserRepositoryImpl> {
            let user_repository = Arc::new(InMemoryUserRepositoryImpl::new());
            let user_application = UserServiceImpl::new(&user_repository);

            user_application
        }

        #[tokio::test]
        async fn create_user_saved() -> anyhow::Result<()> {
            let app_service = setup();
            let id = "1";
            let name = "hoge";
            let email = "mail";
            let create_user = UserData::new(id, name, email);

            app_service.save(create_user.clone()).await?;
            let get_user = app_service.get(id).await?.unwrap();
            assert_eq!(get_user, create_user);

            Ok(())
        }

        #[tokio::test]
        async fn create_same_user_not_saved() -> anyhow::Result<()> {
            let app_service = setup();

            let id = "1";
            let name1 = "hoge";
            let email1 = "fuga";
            let user1 = UserData::new(id, name1, email1);
            let name2 = "sample name";
            let email2 = "abc";
            let user2 = UserData::new(id, name2, email2);
            let created_user = UserData::new(id, name1, email1);

            app_service.save(user1).await?;
            assert!(app_service.save(user2).await.is_err());

            let get_user = app_service.get(id).await?.unwrap();
            assert_eq!(get_user, created_user);

            Ok(())
        }

        #[tokio::test]
        async fn get_not_exist_user_return_none() -> anyhow::Result<()> {
            let app_service = setup();
            let id = "234";

            assert!(app_service.get(id).await?.is_none());

            Ok(())
        }

        #[tokio::test]
        async fn delete_user_return_ok() -> anyhow::Result<()> {
            let app_service = setup();
            let id = "234";
            let name = "delete user";
            let email = "hoge";
            let created_user = UserData::new(id, name, email);

            app_service.save(created_user.clone()).await?;
            assert_eq!(app_service.get(id).await?.unwrap(), created_user);

            app_service.delete(id).await?;
            assert!(app_service.get(id).await?.is_none());

            Ok(())
        }

        #[tokio::test]
        async fn delete_not_exist_user_return_ok() -> anyhow::Result<()> {
            let app_service = setup();
            let id = "234";

            assert!(app_service.get(id).await?.is_none());

            assert!(app_service.delete(id).await.is_ok());

            Ok(())
        }
    }
}
