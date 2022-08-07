use applications::users::UserData;

#[async_trait::async_trait]
pub trait OICDService {
    async fn redirect(&self) -> anyhow::Result<String>;
    async fn verify(&self) -> anyhow::Result<UserData>;
}