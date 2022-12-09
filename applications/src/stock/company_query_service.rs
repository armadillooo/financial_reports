use super::companies::Companies;
use super::company_query_parameters::CompanyQueryParameters;

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(param: CompanyQueryParameters) -> anyhow::Result<Vec<Companies>>;
}
