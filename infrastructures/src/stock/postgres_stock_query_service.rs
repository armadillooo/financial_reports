use applications::stock::{StockData, StockQueryCommand, StockQueryResult, StockQueryService};

pub struct PostgresStockQueryService {}

#[async_trait::async_trait]
impl StockQueryService for PostgresStockQueryService {
    async fn find(&self, param: StockQueryCommand) -> StockQueryResult<Vec<StockData>> {
        unimplemented!()
    }

    async fn find_latest(&self, stock_id: &str) -> StockQueryResult<StockData> {
        unimplemented!()
    }
}
