use crate::auth::OICDClient;
use anyhow::anyhow;
use applications::users::UserData;
use presentation::auth::{OICDData, OICDService};

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
    ) -> anyhow::Result<UserData> {
        let claims = self.oicd_client.verify(verify_info, code, state).await?;

        let email = if let Some(email) = claims.email() {
            email.to_string()
        } else {
            return Err(anyhow!("Email address not found"));
        };
        let name = claims
            .name()
            .ok_or_else(|| anyhow!("User name not found"))?
            .get(claims.locale())
            .ok_or_else(|| anyhow!("User name corresponded to locale not found"))?
            .to_string();
        let id = claims.subject().to_string();

        let user_data = UserData::new(id, name, email);

        Ok(user_data)
    }
}
