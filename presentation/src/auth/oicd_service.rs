use crate::auth::OICDData;
use applications::users::UserData;

#[async_trait::async_trait]
pub trait OICDService {
    async fn redirect(&self) -> OICDData;
    async fn verify(
        &self,
        verify_info: OICDData,
        code: String,
        state: String,
    ) -> anyhow::Result<UserData>;
}
