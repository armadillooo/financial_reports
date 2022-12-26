use std::ops::Deref;

use axum::{extract::FromRequest, http::Request};
use serde::{Deserialize, Serialize};

use crate::{
    common::{ApiError, AppState},
    session::{SessionError, SessionId, SessionItem},
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
impl<S, B> FromRequest<S, B> for LoginUserId
where
    B: Send + 'static,
    S: AppState + Send + Sync,
{
    // エラー時の戻り値の型
    type Rejection = ApiError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // セッションにユーザーIDが登録されている場合はログイン済み
        let key = SessionItem::LoginUserId(LoginUserId::new("".to_string()));
        let session_id = req
            .extensions()
            .get::<SessionId>()
            .ok_or(SessionError::SessionIdRequired)?;

        let SessionItem::LoginUserId(user_id) = state.session_service().item(session_id.clone(), &key).await? else {
            return Err(SessionError::ItemNotFound.into());
        };

        Ok(user_id)
    }
}
