use super::Favorite;

#[async_trait::async_trait]
pub trait FavoriteRepository {
    async fn save(favorite: Favorite) -> anyhow::Result<()>;
    async fn delete(favorite: Favorite) -> anyhow::Result<()>;
    async fn find(user_id: &str) -> anyhow::Result<Vec<Favorite>>;
}
