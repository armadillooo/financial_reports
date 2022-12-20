#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioUpdateCommand {
    pub user_id: String,
    pub stock_id: String,
    pub purchase: Option<i32>,
    pub stock_count: Option<i32>,
}

impl PortfolioUpdateCommand {
    /// コンストラクタ
    pub fn new(
        user_id: String,
        stock_id: String,
        purchase: Option<i32>,
        stock_count: Option<i32>,
    ) -> Self {
        Self {
            user_id,
            stock_id,
            purchase,
            stock_count,
        }
    }
}
