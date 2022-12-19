use chrono::NaiveDate;
use domain::{portfolio::Portfolio, stock::StockId, users::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioData {
    pub user_id: String,
    pub stock_id: String,
    pub stock_count: i32,
    /// 購入価格
    pub purchase: i32,
}

impl From<Portfolio> for PortfolioData {
    fn from(portfolio: Portfolio) -> Self {
        Self {
            user_id: portfolio.user_id.to_string(),
            stock_id: portfolio.stock_id.to_string(),
            stock_count: portfolio.stock_count,
            purchase: portfolio.purchase,
        }
    }
}

impl Into<Portfolio> for PortfolioData {
    fn into(self) -> Portfolio {
        Portfolio {
            user_id: UserId::new(self.user_id),
            stock_id: StockId::new(self.stock_id),
            stock_count: self.stock_count,
            purchase: self.purchase,
        }
    }
}
