use std::sync::{Arc, Mutex};

use domain::{
    portfolio::{Portfolio, PortfolioDomainError, PortfolioDomainResult, PortfolioReposotory},
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
    async fn save(&self, portfolio: Portfolio) -> PortfolioDomainResult<()> {
        self.store.lock().unwrap().push(portfolio);

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> PortfolioDomainResult<()> {
        let mut store = self.store.lock().unwrap();
        let Some(index) = store.iter().position(|target| target.stock_id == *stock_id && target.user_id == *user_id) else {return Ok(())};
        store.remove(index);

        Ok(())
    }

    async fn find_all(&self, user_id: &UserId) -> PortfolioDomainResult<Vec<Portfolio>> {
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

    async fn find(&self, user_id: &UserId, stock_id: &StockId) -> PortfolioDomainResult<Portfolio> {
        let result = self
            .store
            .lock()
            .unwrap()
            .iter()
            .find(|favorite| favorite.user_id == *user_id && favorite.stock_id == *stock_id)
            .map(|found| found.clone())
            .ok_or(PortfolioDomainError::PortfolioNotFound)?;

        Ok(result)
    }
}
