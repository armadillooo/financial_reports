use std::sync::Arc;

use crate::favorite::{FavoriteApplicationResult, FavoriteData, FavoriteService};
use domain::{
    favorite::FavoriteRepository,
    user::{UserDomainService, UserId, UserRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository,
    U: UserRepository,
{
    favorite_repository: Arc<T>,
    user_domain_service: UserDomainService<U>,
}

impl<T, U> FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository,
    U: UserRepository,
{
    /// コンストラクタ
    pub fn new(favorite_repository: &Arc<T>, user_service: UserDomainService<U>) -> Self {
        Self {
            favorite_repository: Arc::clone(favorite_repository),
            user_domain_service: user_service,
        }
    }
}

#[async_trait::async_trait]
impl<T, U> FavoriteService for FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository + Send + Sync,
    U: UserRepository + Send + Sync,
{
    #[tracing::instrument(skip(self), err, ret)]
    async fn get_all(&self, user_id: &str) -> FavoriteApplicationResult<Vec<FavoriteData>> {
        let user_id = UserId::new(user_id.into());
        self.user_domain_service.exists(&user_id).await?;

        let result = self
            .favorite_repository
            .find_all(&user_id)
            .await
            .map(|favorites| {
                favorites
                    .into_iter()
                    .map(|f| FavoriteData::from(f))
                    .collect()
            })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), err)]
    async fn add(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()> {
        let user_id = UserId::new(favorite.user_id.clone());
        self.user_domain_service.exists(&user_id).await?;

        self.favorite_repository.save(favorite.into()).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    async fn remove(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()> {
        let user_id = UserId::new(favorite.user_id.clone());
        self.user_domain_service.exists(&user_id).await?;

        self.favorite_repository.delete(favorite.into()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc};

    use anyhow::anyhow;

    use crate::{
        favorite::{FavoriteService, FavoriteServiceImpl, InmemoryFavoriteRepositoryImpl, FavoriteData, FavoriteApplicationError},
        user::InmemoryUserRepositoryImpl,
    };
    use domain::{user::{UserDomainService, User, UserId, UserEmail, UserName, UserRepository}};

    const USER_ID: &str = "sample user";

    async fn setup() -> impl FavoriteService {
        let sample_user = User::new(UserId::new(USER_ID.to_string()), UserName::new("".to_string()), UserEmail::new("".to_string()));
        let user_repository = Arc::new(InmemoryUserRepositoryImpl::new());
        user_repository.save(sample_user).await.unwrap();
        let favorite_repository = Arc::new(InmemoryFavoriteRepositoryImpl::new());
        let user_domain_service = UserDomainService::new(&user_repository);
        let favorite_service =
        FavoriteServiceImpl::new(&favorite_repository, user_domain_service.clone());

        favorite_service
    }
    
    #[tokio::test]
    async fn add_favorite_success() -> anyhow::Result<()> {
        let service = setup().await;
        let favorite = FavoriteData::new(USER_ID.to_string(), "sample".to_string());
        service.add(favorite).await?;
        
        let result = service.get_all(USER_ID).await?;
        assert!(result[0].user_id == USER_ID);
        assert!(result.len() == 1);

        Ok(())
    }

    #[tokio::test]
    async fn add_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let favorite = FavoriteData::new("not registed user".to_string(), "sample".to_string());
        
        let Err(FavoriteApplicationError::UserNotFound(_)) = service.add(favorite.clone()).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn get_all_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let favorite = FavoriteData::new("not registed user".to_string(), "sample".to_string());
        
        let Err(FavoriteApplicationError::UserNotFound(_)) = service.get_all(&favorite.user_id).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };        

        Ok(())
    }

    #[tokio::test]
    async fn remove_favorite_success() -> anyhow::Result<()> {
        let service = setup().await;
        let favorite = FavoriteData::new(USER_ID.to_string(), "sample".to_string());
        service.add(favorite.clone()).await?;
        
        let result = service.get_all(USER_ID).await?;
        assert!(result.len() == 1);

        service.remove(favorite).await?;
        let result = service.get_all(USER_ID).await?;
        assert!(result.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn remove_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let favorite = FavoriteData::new("not registed user".to_string(), "sample".to_string());
        
        let Err(FavoriteApplicationError::UserNotFound(_)) = service.remove(favorite).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };        

        Ok(())
    }
}
