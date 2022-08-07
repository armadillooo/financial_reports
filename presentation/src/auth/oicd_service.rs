use applications::users::UserData;

#[async_trait::async_trait]
pub trait OICDService {
    async fn redirect(&self, session_id: &str) -> anyhow::Result<String>;
    async fn verify(&self, session_id: &str) -> anyhow::Result<UserData>;
}