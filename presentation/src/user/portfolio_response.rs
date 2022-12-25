use applications::portfolio::PortfolioData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PortfolioResponse {
    user_id: String,
    stock_id: String,
    stock_count: i32,
    purchase: i32,
    market_price: i32,
    latest_date: String,
}

impl From<PortfolioData> for PortfolioResponse {
    fn from(value: PortfolioData) -> Self {
        Self {
            user_id: value.user_id,
            stock_id: value.stock_id,
            stock_count: value.stock_count,
            purchase: value.purchase,
            market_price: value.market_price,
            latest_date: value.latest_date.format("%Y-%m-%d").to_string(),
        }
    }
}
