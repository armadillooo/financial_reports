#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioUpdateCommand {
    pub user_id: String,
    pub stock_id: String,
    pub purchase: Option<i32>,
    pub stock_count: Option<i32>,
}
