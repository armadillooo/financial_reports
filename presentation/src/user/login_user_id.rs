use std::ops::Deref;

use axum::{
    extract::{Extension, FromRequest},
    http::Request,
};
use serde::{Deserialize, Serialize};

use crate::{common::ApiError, session::SessionItemKey};

pub const USER_ID: SessionItemKey<LoginUserId> = SessionItemKey::new("user id");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    S: Send + Sync,
{
    // エラー時の戻り値の型
    type Rejection = ApiError;

    /// ログインが必要なリクエストに対して、Sessionの存在確認を行う
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // Extensinからメモリストアを取得
        let user_id = Extension::<SharedSession>::from_request(req, state)
            .await?
            .read()
            .unwrap()
            .item(&USER_ID)
            .unwarp();

        Ok(user_id)
    }
}
