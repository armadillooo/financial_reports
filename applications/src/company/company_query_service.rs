use super::CompanyData;
use super::CompanyQueryParameters;

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(&self, param: CompanyQueryParameters) -> anyhow::Result<Vec<CompanyData>>;
}
