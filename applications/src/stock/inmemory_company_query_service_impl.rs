use super::companies::Companies;
use super::company_query_parameters::CompanyQueryParameters;
use super::company_query_service::CompanyQueryService;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InmemoryCompanyQueryServiceImpl {
    pub companies: Vec<Companies>,
}

#[async_trait::async_trait]
impl CompanyQueryService for InmemoryCompanyQueryServiceImpl {
    async fn find(param: CompanyQueryParameters) -> anyhow::Result<Vec<Companies>> {
        unimplemented!()
    }
}
