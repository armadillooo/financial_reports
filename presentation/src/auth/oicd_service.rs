use applications::users::UserData;

#[async_trait::async_trait]
pub trait OICDService {
    type VerifyInfo;

    async fn redirect(&self) -> Self::VerifyInfo;
    async fn verify(&self, mut session: Self::VerifyInfo) -> anyhow::Result<UserData>;
}
