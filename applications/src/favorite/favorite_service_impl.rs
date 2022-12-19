use std::sync::Arc;

use crate::favorite::{FavoriteData, FavoriteService};
use domain::favorite::FavoriteRepository;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FavoriteServiceImpl<T>
where
    T: FavoriteRepository,
{
    favorite_repository: Arc<T>,
}

#[async_trait::async_trait]
impl<T> FavoriteService for FavoriteServiceImpl<T>
where
    T: FavoriteRepository + Send + Sync,
{
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<FavoriteData>> {
        let result = self
            .favorite_repository
            .find(user_id)
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
        self.favorite_repository.save(favorite.into()).await
    }

    async fn remove(&self, favorite: FavoriteData) -> anyhow::Result<()> {
        self.favorite_repository.delete(favorite.into()).await
    }
}
