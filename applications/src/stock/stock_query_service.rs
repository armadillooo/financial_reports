use super::stock_data::StockData;
use super::stock_query_command::StockQueryCommand;

#[async_trait::async_trait]
pub trait StockQueryService {
    async fn find(&self, param: StockQueryCommand) -> anyhow::Result<Vec<StockData>>;
}
