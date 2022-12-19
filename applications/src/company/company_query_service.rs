use super::CompanyData;
use super::CompanyQueryCommand;

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(&self, param: CompanyQueryCommand) -> anyhow::Result<Vec<CompanyData>>;
}
