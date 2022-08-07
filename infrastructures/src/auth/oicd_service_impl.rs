use std::sync::Arc;

use super::oicd_client::OICDClient;
use applications::users::UserData;
use presentation::auth::OICDService;
use presentation::session::SessionRepository;

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
    T: SessionRepository + Send + Sync,
{
    async fn redirect(&self) -> anyhow::Result<String> {
        let oicd_info = self.oicd_client.redirect_url().await;

        Ok(oicd_info.auth_url.to_string())
    }

    async fn verify(&self) -> anyhow::Result<UserData> {
        unimplemented!()
    }
}
