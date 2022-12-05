use super::company_query_parameters::CompanyQueryParameters;
use super::companies::Companies;

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(param: CompanyQueryParameters) -> Vec<Companies>;
}