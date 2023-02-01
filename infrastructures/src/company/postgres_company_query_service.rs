use applications::company::{
    CompanyData, CompanyQueryCommand, CompanyQueryResult, CompanyQueryService,
};

pub struct PostgresCompanyQueryService {}

#[async_trait::async_trait]
impl CompanyQueryService for PostgresCompanyQueryService {
    async fn find(&self, param: CompanyQueryCommand) -> CompanyQueryResult<Vec<CompanyData>> {
        unimplemented!()
    }

    async fn find_by_id(&self, stock_id: &str) -> CompanyQueryResult<CompanyData> {
        unimplemented!()
    }

    async fn find_list(&self, stock_id_list: Vec<String>) -> CompanyQueryResult<Vec<CompanyData>> {
        unimplemented!()
    }
}
