use std::sync::{Arc, Mutex};

use domain::{
    favorite::{Favorite, FavoriteRepository},
    users::UserId,
};

#[derive(Debug, Clone, Default)]
pub struct InmemoryFavoriteRepositoryImpl {
    pub store: Arc<Mutex<Vec<Favorite>>>,
}

impl InmemoryFavoriteRepositoryImpl {
    /// コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl FavoriteRepository for InmemoryFavoriteRepositoryImpl {
    async fn save(&self, favorite: Favorite) -> anyhow::Result<()> {
        self.store.lock().unwrap().push(favorite);

        Ok(())
    }

    async fn delete(&self, favorite: Favorite) -> anyhow::Result<()> {
        let mut store = self.store.lock().unwrap();
        let Some(index) = store.iter().position(|target| target == &favorite) else {return Ok(())};
        store.remove(index);

        Ok(())
    }

    async fn find_all(&self, user_id: &UserId) -> anyhow::Result<Vec<Favorite>> {
        let result = self
            .store
            .lock()
            .unwrap()
            .to_vec()
            .into_iter()
            .filter(|favorite| favorite.user_id == *user_id);
        let result = result.collect::<Vec<Favorite>>();

        Ok(result)
    }
}
