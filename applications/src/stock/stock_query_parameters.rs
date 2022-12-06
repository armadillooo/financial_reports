use chrono::NaiveDate;

pub struct StockQueryParameters {
    pub name: Option<String>,
    pub id: Option<String>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub order_by: Option<String>,
    pub page: Option<i32>,
}
