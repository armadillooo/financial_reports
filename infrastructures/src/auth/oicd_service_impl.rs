use std::sync::Arc;

use anyhow::anyhow;

use crate::auth::OICDClient;
use crate::session::SessionDataImpl;
use applications::users::UserData;
use presentation::auth::OICDService;
use presentation::session::{SessionData, SessionRepository};

pub struct OICDserviceImpl<T>
where
    T: SessionRepository,
    // <T>はどんなオブジェクトが入るかわからないため, Send + Syncを実装していない可能性がある
    // そのため、トレイト境界を設定する
{
    oicd_client: OICDClient,
    session_repository: Arc<T>,
}

impl<T> OICDserviceImpl<T>
where
    T: SessionRepository,
{
    /// コンストラクタ
    pub fn new(oicd_client: OICDClient, sessioin_repository: &Arc<T>) -> Self {
        Self {
            oicd_client,
            session_repository: Arc::clone(sessioin_repository),
        }
    }
}

#[async_trait::async_trait]
impl<T> OICDService for OICDserviceImpl<T>
where
    T: SessionRepository<Data = SessionDataImpl> + Send + Sync,
{
    /// ユーザーをリダイレクトさせる
    async fn redirect(&self, session_id: &str) -> anyhow::Result<String> {
        let oicd_info = self.oicd_client.redirect_info();
        let redirect_url = oicd_info.auth_url.clone();

        let mut session = if let Some(session) = self.session_repository.find(session_id).await? {
            session
        } else {
            SessionDataImpl::new()
        };

        session.set_oicd_info(oicd_info);
        self.session_repository.save(session).await?;

        Ok(redirect_url.to_string())
    }

    /// 認証情報の検証
    async fn verify(&self, session_id: &str) -> anyhow::Result<UserData> {
        let session = self
            .session_repository
            .find(session_id)
            .await?
            .ok_or_else(|| anyhow!("There is no session"))?;

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
