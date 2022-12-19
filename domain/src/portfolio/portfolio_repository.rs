use super::Portfolio;
use crate::{stock::StockId, users::UserId};

#[async_trait::async_trait]
pub trait PortfolioReposotory {
    async fn save(&self, portfolio: Portfolio) -> anyhow::Result<()>;
    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> anyhow::Result<()>;
    async fn find_all(&self, user_id: &UserId) -> anyhow::Result<Vec<Portfolio>>;
    async fn find(&self, user_id: &UserId, stock_id: &StockId)
        -> anyhow::Result<Option<Portfolio>>;
}
