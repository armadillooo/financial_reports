#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PortfolioUpdateCommand {
    pub stock_id: String,
    pub purchase: Option<i32>,
    pub stock_count: Option<i32>,
}
