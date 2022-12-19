#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioData {
    pub user_id: String,
    pub stock_id: String,
    pub stock_count: i32,
    /// 購入価格
    pub purchase: i32,
    /// 時価
    pub market_price: i32,
}
