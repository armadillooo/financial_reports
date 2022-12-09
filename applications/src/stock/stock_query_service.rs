use super::stock_data::StockData;
use super::stock_query_parameters::StockQueryParameters;

#[async_trait::async_trait]
pub trait StockQueryService {
    async fn find(&self, param: StockQueryParameters) -> anyhow::Result<Vec<StockData>>;
}
