use crate::company::{CompanyData, CompanyQueryCommand, CompanyQueryResult};

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(&self, param: CompanyQueryCommand) -> CompanyQueryResult<Vec<CompanyData>>;
    async fn find_by_id(&self, stock_id: &str) -> CompanyQueryResult<CompanyData>;
    async fn find_list(
        &self,
        stock_id_list: Vec<String>,
    ) -> CompanyQueryResult<Vec<CompanyData>>;
}
