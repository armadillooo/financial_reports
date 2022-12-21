use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StockQueryCommand {
    pub stock_id: Option<String>,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

impl StockQueryCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
