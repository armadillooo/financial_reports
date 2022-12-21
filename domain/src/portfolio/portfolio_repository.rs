use crate::{
    portfolio::{Portfolio, PortfolioDomainResult},
    stock::StockId,
    users::UserId,
};

#[async_trait::async_trait]
pub trait PortfolioReposotory {
    async fn save(&self, portfolio: Portfolio) -> PortfolioDomainResult<()>;
    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> PortfolioDomainResult<()>;
    async fn find_all(&self, user_id: &UserId) -> PortfolioDomainResult<Vec<Portfolio>>;
    async fn find(
        &self,
        user_id: &UserId,
        stock_id: &StockId,
    ) -> PortfolioDomainResult<Portfolio>;
}
