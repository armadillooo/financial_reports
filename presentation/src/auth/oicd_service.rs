use crate::auth::{OICDData, OICDResult};
use applications::user::UserData;

#[async_trait::async_trait]
pub trait OICDService {
    async fn redirect(&self) -> OICDData;
    async fn verify(
        &self,
        verify_info: OICDData,
        code: String,
        state: String,
    ) -> OICDResult<UserData>;
}
