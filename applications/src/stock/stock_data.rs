use chrono::NaiveDate;

use super::stock_id::StockId;

/// 株価情報
pub struct StockData {
    pub stock_id: StockId,
    /// 日付
    pub date: NaiveDate,
    /// 出来高
    pub volume: f32,
    /// 始値
    pub start_price: f32,
    /// 終値
    pub end_price: f32,
    /// 高値
    pub high_price: f32,
    /// 安値
    pub low_price: f32,
}
