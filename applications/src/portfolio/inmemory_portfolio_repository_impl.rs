use std::sync::{Arc, Mutex};

use domain::{
    portfolio::{Portfolio, PortfolioReposotory},
    stock::StockId,
    users::UserId,
};

#[derive(Debug, Clone, Default)]
pub struct InmemoryPortfolioRepositoryImpl {
    pub store: Arc<Mutex<Vec<Portfolio>>>,
}

impl InmemoryPortfolioRepositoryImpl {
    /// コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl PortfolioReposotory for InmemoryPortfolioRepositoryImpl {
    async fn save(&self, portfolio: Portfolio) -> anyhow::Result<()> {
        self.store.lock().unwrap().push(portfolio);

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> anyhow::Result<()> {
        let mut store = self.store.lock().unwrap();
        let Some(index) = store.iter().position(|target| target.stock_id == *stock_id && target.user_id == *user_id) else {return Ok(())};
        store.remove(index);

        Ok(())
    }

    async fn find_all(&self, user_id: &UserId) -> anyhow::Result<Vec<Portfolio>> {
        let result = self
            .store
            .lock()
            .unwrap()
            .to_vec()
            .into_iter()
            .filter(|favorite| favorite.user_id == *user_id);
        let result = result.collect::<Vec<Portfolio>>();

        Ok(result)
    }

    async fn find(
        &self,
        user_id: &UserId,
        stock_id: &StockId,
    ) -> anyhow::Result<Option<Portfolio>> {
        let result = self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|favorite| favorite.user_id == *user_id && favorite.stock_id == *stock_id)
            .map(|found| found.clone());

        Ok(result)
    }
}
