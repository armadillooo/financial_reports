use super::FavoriteData;

#[async_trait::async_trait]
pub trait FavoriteService {
    async fn add(&self, favorite: FavoriteData) -> anyhow::Result<()>;
    async fn remove(&self, favoiret: FavoriteData) -> anyhow::Result<()>;
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<FavoriteData>>;
}
