use super::stock_id::StockId;

pub struct StockData {
    pub symbol: StockId,
    pub volume: f32,
    pub start_price: f32,
    pub end_price: f32,
    pub high_price: f32,
    pub low_price: f32,
}
