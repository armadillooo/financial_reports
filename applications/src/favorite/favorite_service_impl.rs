use std::sync::Arc;

use crate::favorite::{FavoriteData, FavoriteService};
use anyhow::anyhow;
use domain::{
    favorite::FavoriteRepository,
    users::{UserId, UserRepository, UserService},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository,
    U: UserRepository,
{
    favorite_repository: Arc<T>,
    user_service: UserService<U>,
}

#[async_trait::async_trait]
impl<T, U> FavoriteService for FavoriteServiceImpl<T, U>
where
    T: FavoriteRepository + Send + Sync,
    U: UserRepository + Send + Sync,
{
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<FavoriteData>> {
        let result = self
            .favorite_repository
            .find(&user_id)
            .await
            .map(|favorites| {
                favorites
                    .into_iter()
                    .map(|f| FavoriteData::from(f))
                    .collect()
            });

        result
    }

    async fn add(&self, favorite: FavoriteData) -> anyhow::Result<()> {
        let user_id = UserId::new(favorite.user_id.clone());
        if self.user_service.exists(&user_id).await == false {
            return Err(anyhow!("user not exists"));
        };

        self.favorite_repository.save(favorite.into()).await
    }

    async fn remove(&self, favorite: FavoriteData) -> anyhow::Result<()> {
        self.favorite_repository.delete(favorite.into()).await
    }
}
