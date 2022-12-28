use chrono::NaiveDate;
/// 株価情報
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StockData {
    pub stock_id: String,
    /// 日付
    pub date: NaiveDate,
    /// 出来高
    pub volume: i32,
    /// 始値
    pub start_price: i32,
    /// 終値
    pub end_price: i32,
    /// 高値
    pub high_price: i32,
    /// 安値
    pub low_price: i32,
}

impl StockData {
    pub fn new() -> Self {
        Self::default()
    }
}
