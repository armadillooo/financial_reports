use super::stock_data::StockData;
use super::stock_query_parameters::StockQueryParameters;
use super::stock_query_service::StockQueryService;

pub struct InmemoryStockQueryServiceImpl {}

#[async_trait::async_trait]
impl StockQueryService for InmemoryStockQueryServiceImpl {
    async fn find(param: StockQueryParameters) -> Vec<StockData> {
        unimplemented!()
    }
}
