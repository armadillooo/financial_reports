use super::FavoriteData;

#[async_trait::async_trait]
pub trait FavoriteService {
    async fn add(favorite: FavoriteData) -> anyhow::Result<()>;
    async fn remove(favoiret: FavoriteData) -> anyhow::Result<()>;
    async fn get_all(user_id: &str) -> anyhow::Result<Vec<FavoriteData>>;
}
