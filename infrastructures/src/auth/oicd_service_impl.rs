use crate::auth::OICDClient;
use applications::users::UserData;
use presentation::auth::{OICDData, OICDError, OICDResult, OICDService};

#[derive(Debug, Clone)]
pub struct OICDserviceImpl
// <T>はどんなオブジェクトが入るかわからないため, Send + Syncを実装していない可能性がある
// そのため、トレイト境界を設定する
{
    oicd_client: OICDClient,
}

impl OICDserviceImpl {
    /// コンストラクタ
    pub fn new(oicd_client: OICDClient) -> Self {
        Self { oicd_client }
    }
}

#[async_trait::async_trait]
impl OICDService for OICDserviceImpl {
    /// ユーザーをリダイレクトさせる
    async fn redirect(&self) -> OICDData {
        self.oicd_client.redirect_info()
    }

    /// 認証情報の検証
    async fn verify(
        &self,
        verify_info: OICDData,
        code: String,
        state: String,
    ) -> OICDResult<UserData> {
        let claims = self
            .oicd_client
            .verify(verify_info, code, state)
            .await
            .map_err(|_| OICDError::VerifyError)?;

        let mut email = claims
            .email()
            .ok_or(OICDError::NotRegisterdEmail)?
            .to_string();
        let email_domain_offset = email.find('@').unwrap_or(email.len());
        let name: String = email.drain(..email_domain_offset).collect();
        let id = claims.subject().to_string();

        Ok(UserData::new(id, name, email))
    }
}
