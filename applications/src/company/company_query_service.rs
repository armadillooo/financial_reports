use crate::company::{CompanyData, CompanyQueryCommand, CompanyQueryResult};

#[async_trait::async_trait]
pub trait CompanyQueryService {
    async fn find(&self, param: CompanyQueryCommand) -> CompanyQueryResult<Vec<CompanyData>>;
}
