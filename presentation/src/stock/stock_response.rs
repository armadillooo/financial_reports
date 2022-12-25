use applications::stock::StockData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StockResponse {
    stock_id: String,
    date: String,
    volume: i32,
    start_price: i32,
    end_price: i32,
    high_price: i32,
    low_price: i32,
}

impl From<StockData> for StockResponse {
    fn from(value: StockData) -> Self {
        Self {
            stock_id: value.stock_id.to_string(),
            date: value.date.format("%Y-%m-%d").to_string(),
            volume: value.volume,
            start_price: value.start_price,
            end_price: value.end_price,
            high_price: value.high_price,
            low_price: value.low_price,
        }
    }
}
