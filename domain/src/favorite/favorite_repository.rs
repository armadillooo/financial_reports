use crate::{favorite::Favorite, users::UserId};

#[async_trait::async_trait]
pub trait FavoriteRepository {
    async fn save(&self, favorite: Favorite) -> anyhow::Result<()>;
    async fn delete(&self, favorite: Favorite) -> anyhow::Result<()>;
    async fn find_all(&self, user_id: &UserId) -> anyhow::Result<Vec<Favorite>>;
}
