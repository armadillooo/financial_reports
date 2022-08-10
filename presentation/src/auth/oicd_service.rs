use applications::users::UserData;
use crate::session::SessionData;

#[async_trait::async_trait]
pub trait OICDService {
    type Session: SessionData;

    async fn redirect(&self, mut session: Self::Session) -> anyhow::Result<String>;
    async fn verify(&self, mut session: Self::Session) -> anyhow::Result<UserData>;
}