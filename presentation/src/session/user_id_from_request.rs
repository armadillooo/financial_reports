//! Http RequestからセッションIDを抽出する
use axum::{
    extract::{Extension, FromRequest, Json, RequestParts},
    http::StatusCode,
};

use crate::{
    common::{self, ApiError, JsonBuilder},
    session::SharedSession,
    user::{LoginedUserId, USER_ID},
};

// 認証が必要なAPIのハンドラに引数で渡すことで
// 認証のチェックを自動で行う
#[axum::async_trait]
impl<T> FromRequest<T> for LoginedUserId
where
    T: Send,
{
    // エラー時の戻り値の型
    type Rejection = common::Rejection;

    /// ログインが必要なリクエストに対して、Sessionの存在確認を行う
    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        // Extensinからメモリストアを取得
        let user_id = Extension::<SharedSession>::from_request(req)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        JsonBuilder::new()
                            .add(ApiError {
                                message: "Internal server error occured",
                            })
                            .build(),
                    ),
                )
            })?
            .read()
            .unwrap()
            .item(&USER_ID)
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        JsonBuilder::new()
                            .add(ApiError {
                                message: "Authentication required",
                            })
                            .build(),
                    ),
                )
            })?;

        Ok(user_id)
    }
}
