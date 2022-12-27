use std::ops::Deref;

use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};

use crate::{
    auth::OICDError,
    common::{ApiError, AppState},
    session::{SessionId, SessionItem},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginUserId(String);

impl LoginUserId {
    /// コンストラクタ
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Into<String> for LoginUserId {
    fn into(self) -> String {
        self.0
    }
}

impl Deref for LoginUserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 認証が必要なAPIのハンドラに引数で渡すことで
// 認証のチェックを自動で行う
#[axum::async_trait]
impl<S> FromRequestParts<S> for LoginUserId
where
    S: AppState + Send + Sync,
{
    // エラー時の戻り値の型
    type Rejection = ApiError;

    #[tracing::instrument(skip(parts, state), err, ret)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // セッションにユーザーIDが登録されている場合はログイン済み
        let key = SessionItem::LoginUserId(LoginUserId::new("".to_string()));
        let session_id = parts
            .extensions
            .get::<SessionId>()
            .expect("there is no SessionId extension. please add session manage layer before LoginUserId extractor");

        let Some(SessionItem::LoginUserId(user_id)) = state.session_service().find_item(session_id.clone(), &key).await? else {
            return Err(OICDError::AuthenticationRequired.into());
        };

        Ok(user_id)
    }
}
