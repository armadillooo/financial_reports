use super::{portfolio_data::PortfolioData, portfolio_update_command::PortfolioUpdateCommand};

#[async_trait::async_trait]
pub trait PortfolioService {
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<PortfolioData>>;
    async fn remove(&self, user_id: &str, stock_id: &str) -> anyhow::Result<()>;
    async fn update(&self, update_command: PortfolioUpdateCommand) -> anyhow::Result<()>;
    async fn add(&self, portfolio: PortfolioData) -> anyhow::Result<()>;
}
