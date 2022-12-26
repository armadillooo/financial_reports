use std::sync::Arc;

use crate::favorite::{FavoriteData, FavoriteService, FavoriteApplicationResult};
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
    user_service: UserDomainService<U>,
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
            user_service,
        }
    }
}

#[async_trait::async_trait]
impl<T, U> FavoriteService for FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository + Send + Sync,
    U: UserRepository + Send + Sync,
{
    async fn get_all(&self, user_id: &str) -> FavoriteApplicationResult<Vec<FavoriteData>> {
        let user_id = UserId::new(user_id.into());

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

    async fn add(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()> {
        let user_id = UserId::new(favorite.user_id.clone());
        self.user_service.exists(&user_id).await?;
        self.favorite_repository.save(favorite.into()).await?;

        Ok(())
    }

    async fn remove(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()> {
        self.favorite_repository.delete(favorite.into()).await?;

        Ok(())
    }
}
