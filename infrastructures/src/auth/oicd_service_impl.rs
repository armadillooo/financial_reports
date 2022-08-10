use anyhow::anyhow;

use crate::auth::OICDClient;
use crate::session::SessionDataImpl;
use applications::users::UserData;
use presentation::auth::OICDService;
use presentation::session::{SessionData, SessionService};

pub struct OICDserviceImpl<T>
where
    T: SessionService,
    // <T>はどんなオブジェクトが入るかわからないため, Send + Syncを実装していない可能性がある
    // そのため、トレイト境界を設定する
{
    oicd_client: OICDClient,
    session_service: T,
}

impl<T> OICDserviceImpl<T>
where
    T: SessionService,
{
    /// コンストラクタ
    pub fn new(oicd_client: OICDClient, session_service: T) -> Self {
        Self {
            oicd_client,
            session_service,
        }
    }
}

#[async_trait::async_trait]
impl<T> OICDService for OICDserviceImpl<T>
where
    T: SessionService<Data = SessionDataImpl> + Send + Sync,
{
    type Session = SessionDataImpl;

    /// ユーザーをリダイレクトさせる
    async fn redirect(&self, mut session: Self::Session) -> anyhow::Result<String> {
        let oicd_info = self.oicd_client.redirect_info();
        let redirect_url = oicd_info.auth_url.clone();

        session.set_oicd_info(oicd_info);
        self.session_service.save(session).await?;

        Ok(redirect_url.to_string())
    }

    /// 認証情報の検証
    async fn verify(&self, session: Self::Session) -> anyhow::Result<UserData> {
        let oicd_info = session
            .oicd_info()
            .ok_or_else(|| anyhow!("Could not find authentication info from the session"))?;

        let claims = self.oicd_client.verify(oicd_info).await?;

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
