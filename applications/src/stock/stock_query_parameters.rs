use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StockQueryParameters {
    pub stock_id: String,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

impl StockQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
