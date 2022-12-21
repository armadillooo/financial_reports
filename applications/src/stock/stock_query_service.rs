use crate::stock::{StockData, StockQueryCommand, StockQueryResult};

#[async_trait::async_trait]
pub trait StockQueryService {
    async fn find(&self, param: StockQueryCommand) -> StockQueryResult<Vec<StockData>>;
    async fn find_latest(&self, stock_id: &str) -> StockQueryResult<StockData>;
}
