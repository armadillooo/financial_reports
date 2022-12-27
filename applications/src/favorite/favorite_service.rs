use crate::favorite::{FavoriteApplicationResult, FavoriteData};

#[async_trait::async_trait]
pub trait FavoriteService {
    async fn add(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()>;
    async fn remove(&self, favorite: FavoriteData) -> FavoriteApplicationResult<()>;
    async fn get_all(&self, user_id: &str) -> FavoriteApplicationResult<Vec<FavoriteData>>;
}
