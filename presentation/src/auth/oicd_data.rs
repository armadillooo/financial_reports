use openidconnect::{CsrfToken, Nonce, PkceCodeVerifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OICDData {
    pub auth_url: String,
    pub pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub nonce: Nonce,
}

impl OICDData {
    /// コンストラクタ
    pub fn new() -> Self {
        Self {
            auth_url: String::default(),
            pkce_verifier: PkceCodeVerifier::new("".to_string()),
            csrf_token: CsrfToken::new("".to_string()),
            nonce: Nonce::new("".to_string()),
        }
    }
}
