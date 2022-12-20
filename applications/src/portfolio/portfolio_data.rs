use chrono::NaiveDate;
use domain::{portfolio::Portfolio, stock::StockId, users::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioData {
    pub user_id: String,
    pub stock_id: String,
    /// 購入株数
    pub stock_count: i32,
    /// 購入価格
    pub purchase: i32,
    /// 時価
    pub market_price: i32,
    /// 時価更新日
    pub latest_date: NaiveDate,
}

impl PortfolioData {
    /// コンストラクタ
    pub fn new(user_id: String, stock_id: String) -> Self {
        Self {
            user_id,
            stock_id,
            ..Default::default()
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
