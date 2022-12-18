use super::Portfolio;
use crate::users::UserId;

#[async_trait::async_trait]
pub trait PortfolioReposotory {
    async fn save(portfolio: Portfolio) -> anyhow::Result<()>;
    async fn delete(portfolio: Portfolio) -> anyhow::Result<()>;
    async fn find(user_id: &UserId) -> anyhow::Result<Vec<Portfolio>>;
}
