//! Http RequestからセッションIDを抽出する
use axum::{
    extract::{Extension, FromRequest},
    http::{StatusCode, Request},
};

use crate::{
    common::{internal_error, ApiResponse, ErrorResponse, JsonBuilder},
    session::SharedSession,
    user::{LoginedUserId, USER_ID},
};

// 認証が必要なAPIのハンドラに引数で渡すことで
// 認証のチェックを自動で行う
#[axum::async_trait]
impl<S, B> FromRequest<S, B> for LoginedUserId
where
    B: Send + 'static,
    S: Send + Sync,
{
    // エラー時の戻り値の型
    type Rejection = ApiResponse;

    /// ログインが必要なリクエストに対して、Sessionの存在確認を行う
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // Extensinからメモリストアを取得
        let user_id = Extension::<SharedSession>::from_request(req, state)
            .await
            .map_err(|_| internal_error())?
            .read()
            .unwrap()
            .item(&USER_ID)
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    JsonBuilder::new()
                        .add(ErrorResponse {
                            message: "Authentication required",
                        })
                        .build(),
                )
            })?;

        Ok(user_id)
    }
}
