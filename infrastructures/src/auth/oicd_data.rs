use openidconnect::url::Url;
use openidconnect::{CsrfToken, Nonce, PkceCodeVerifier};

pub struct OICDData {
    pub auth_url: Url,
    pub pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub nonce: Nonce,
}
