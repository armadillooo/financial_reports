use crate::portfolio::{PortfolioData, PortfolioService, PortfolioUpdateCommand};

pub struct PortfolioServieImpl {}

#[async_trait::async_trait]
impl PortfolioService for PortfolioServieImpl {
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<PortfolioData>> {
        unimplemented!()
    }

    async fn remove(&self, user_id: &str, stock_id: &str) -> anyhow::Result<()> {
        unimplemented!()
    }

    async fn update(&self, update_command: PortfolioUpdateCommand) -> anyhow::Result<PortfolioData> {
        unimplemented!()
    }

    async fn add(&self, portfolio: PortfolioData) -> anyhow::Result<()> {
        unimplemented!()
    }
}
