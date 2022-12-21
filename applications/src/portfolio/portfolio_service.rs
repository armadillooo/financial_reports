use crate::portfolio::{PortfoliApplicationResult, PortfolioData, PortfolioUpdateCommand};

#[async_trait::async_trait]
pub trait PortfolioService {
    async fn get_all(&self, user_id: &str) -> PortfoliApplicationResult<Vec<PortfolioData>>;
    async fn remove(&self, user_id: &str, stock_id: &str) -> PortfoliApplicationResult<()>;
    async fn update(&self, update_command: PortfolioUpdateCommand)
        -> PortfoliApplicationResult<()>;
    async fn add(&self, portfolio: PortfolioData) -> PortfoliApplicationResult<()>;
}
