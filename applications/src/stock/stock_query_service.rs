use crate::stock::{StockData, StockQueryCommand};

#[async_trait::async_trait]
pub trait StockQueryService {
    async fn find(&self, param: StockQueryCommand) -> anyhow::Result<Vec<StockData>>;
    async fn find_latest(&self, stock_id: &str) -> anyhow::Result<Option<StockData>>;
}
