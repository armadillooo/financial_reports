use domain::{
    portfolio::{Portfolio, PortfolioDomainResult, PortfolioReposotory},
    stock::StockId,
    user::UserId,
};

pub struct PostgresPortfolioRepository {}

#[async_trait::async_trait]
impl PortfolioReposotory for PostgresPortfolioRepository {
    async fn save(&self, portfolio: Portfolio) -> PortfolioDomainResult<()> {
        unimplemented!()
    }

    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> PortfolioDomainResult<()> {
        unimplemented!()
    }

    async fn find_all(&self, user_id: &UserId) -> PortfolioDomainResult<Vec<Portfolio>> {
        unimplemented!()
    }
    async fn find(
        &self,
        user_id: &UserId,
        stock_id: &StockId,
    ) -> PortfolioDomainResult<Option<Portfolio>> {
        unimplemented!()
    }
}
