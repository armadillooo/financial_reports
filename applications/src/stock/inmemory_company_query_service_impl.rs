use super::companies::Companies;
use super::company_query_parameters::CompanyQueryParameters;
use super::company_query_service::CompanyQueryService;

pub struct InmemoryCompanyQueryServiceImpl {

}

#[async_trait::async_trait]
impl CompanyQueryService for InmemoryCompanyQueryServiceImpl {
    async fn find(param: CompanyQueryParameters) ->  Vec<Companies> {
        unimplemented!()
    }
}