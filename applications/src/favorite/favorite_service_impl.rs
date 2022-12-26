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
    use std::sync::Arc;

    use crate::{
        favorite::{FavoriteService, FavoriteServiceImpl, InmemoryFavoriteRepositoryImpl},
        user::InmemoryUserRepositoryImpl,
    };
    use domain::user::UserDomainService;

    fn setup() -> impl FavoriteService {
        let user_repository = Arc::new(InmemoryUserRepositoryImpl::new());
        let favorite_repository = Arc::new(InmemoryFavoriteRepositoryImpl::new());
        let user_domain_service = UserDomainService::new(&user_repository);
        let favorite_service =
            FavoriteServiceImpl::new(&favorite_repository, user_domain_service.clone());

        favorite_service
    }

    #[tokio::test]
    async fn get_all_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        unimplemented!()
    }

    #[tokio::test]
    async fn get_all_success() -> anyhow::Result<()> {
        unimplemented!()
    }

    #[tokio::test]
    async fn add_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        unimplemented!()
    }

    #[tokio::test]
    async fn add_favorite_success() -> anyhow::Result<()> {
        unimplemented!()
    }

    #[tokio::test]
    async fn remove_notexist_user_favorite_return_err() -> anyhow::Result<()> {
        unimplemented!()
    }

    #[tokio::test]
    async fn remove_favorite_success() -> anyhow::Result<()> {
        unimplemented!()
    }
}
