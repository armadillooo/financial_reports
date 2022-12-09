use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockQueryParameters {
    pub stock_id: String,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}
