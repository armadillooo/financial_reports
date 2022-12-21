use crate::{
    favorite::{Favorite, FavoriteDomainResult},
    users::UserId,
};

#[async_trait::async_trait]
pub trait FavoriteRepository {
    async fn save(&self, favorite: Favorite) -> FavoriteDomainResult<()>;
    async fn delete(&self, favorite: Favorite) -> FavoriteDomainResult<()>;
    async fn find_all(&self, user_id: &UserId) -> FavoriteDomainResult<Vec<Favorite>>;
}
