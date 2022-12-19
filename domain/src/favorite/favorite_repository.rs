use super::Favorite;

#[async_trait::async_trait]
pub trait FavoriteRepository {
    async fn save(&self, favorite: Favorite) -> anyhow::Result<()>;
    async fn delete(&self, favorite: Favorite) -> anyhow::Result<()>;
    async fn find(&self, user_id: &str) -> anyhow::Result<Vec<Favorite>>;
}
