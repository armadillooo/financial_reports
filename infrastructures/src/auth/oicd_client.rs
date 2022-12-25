use anyhow::anyhow;
use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreUserInfoClaims,
};
use openidconnect::{
    reqwest::async_http_client, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl,
    Nonce, PkceCodeChallenge, RedirectUrl, Scope,
};
use openidconnect::{AccessTokenHash, OAuth2TokenResponse, TokenResponse};

use presentation::auth::OICDData;

#[derive(Debug, Clone)]
pub struct OICDClient {
    client: CoreClient,
}

impl OICDClient {
    /// コンストラクタ
    pub async fn new(
        isuser_url: String,
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> anyhow::Result<Self> {
        let provider_metadata =
            CoreProviderMetadata::discover_async(IssuerUrl::new(isuser_url)?, async_http_client)
                .await?;

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);

        Ok(Self { client })
    }

    /// リダイレクト先情報の取得
    pub fn redirect_info(&self) -> OICDData {
        // Generate a PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the full authorization URL
        let (auth_url, csrf_token, nonce) = self
            .client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            // Set the desired scopes
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ))
            .set_pkce_challenge(pkce_challenge)
            .url();

        OICDData {
            auth_url: auth_url.to_string(),
            pkce_verifier: pkce_verifier,
            csrf_token: csrf_token,
            nonce: nonce,
        }
    }

    /// 検証
    #[tracing::instrument(skip(self), err, ret)]
    pub async fn verify(
        &self,
        oicd_info: OICDData,
        code: String,
        state: String,
    ) -> anyhow::Result<CoreUserInfoClaims> {
        if oicd_info.csrf_token.secret() != &state {
            return Err(anyhow!("state verify failed"));
        }

        // Exchange it for an access token and ID token
        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(oicd_info.pkce_verifier)
            .request_async(async_http_client)
            .await?;

        // Extranct the ID token claims after verifying its authenticity and nonce
        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Server did not return an ID token"))?;
        let claims = id_token.claims(&self.client.id_token_verifier(), &oicd_info.nonce)?;

        // Verify the access token hash to ensure that the access token hasn't been substitude for another user's
        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg()?,
            )?;

            if actual_access_token_hash != *expected_access_token_hash {
                return Err(anyhow!("Invalid access token"));
            }
        }

        let user_info = self
            .client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(|err| anyhow!("No user info endpoint: {:?}", err))?
            .request_async(async_http_client)
            .await
            .map_err(|err| anyhow!("Failed requesting user info: {:?}", err))?;

        Ok(user_info)
    }
}
