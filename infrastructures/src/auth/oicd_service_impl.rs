use crate::auth::{OICDClient, OICDData};
use applications::users::UserData;
use presentation::auth::OICDService;

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
    type VerifyInfo = OICDData;

    /// ユーザーをリダイレクトさせる
    async fn redirect(&self) -> Self::VerifyInfo {
        self.oicd_client.redirect_info()
    }

    /// 認証情報の検証
    async fn verify(&self, info: Self::VerifyInfo) -> anyhow::Result<UserData> {
        let claims = self.oicd_client.verify(info).await?;

        let user_name = if let Some(email) = claims.email() {
            email.to_string()
        } else {
            "ゲスト".to_string()
        };
        let user_id = claims.subject().to_string();
        let user_data = UserData::new(user_id, user_name);

        Ok(user_data)
    }
}
