use super::Portfolio;
use crate::users::UserId;

#[async_trait::async_trait]
pub trait PortfolioReposotory {
    async fn save(&self, portfolio: Portfolio) -> anyhow::Result<()>;
    async fn delete(&self, portfolio: Portfolio) -> anyhow::Result<()>;
    async fn find(&self, user_id: &UserId) -> anyhow::Result<Vec<Portfolio>>;
}
