use super::{portfolio_data::PortfolioData, portfolio_update_command::PortfolioUpdateCommand};

#[async_trait::async_trait]
pub trait PortfolioService {
    async fn get_all(user_id: &str) -> anyhow::Result<Vec<PortfolioData>>;
    async fn delete(user_id: &str, stock_id: &str) -> anyhow::Result<()>;
    async fn update(update_command: PortfolioUpdateCommand) -> anyhow::Result<PortfolioData>;
}
