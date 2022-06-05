//! Http RequestからセッションIDを抽出する
use axum::{
    extract::{Extension, FromRequest, Json, RequestParts, TypedHeader},
    headers::Cookie,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::session::{database::Store, SESSION_USER_ID_KEY};

/// Sessionに保存されるユーザーID
#[derive(Deserialize, Serialize)]
pub struct UserId(pub i32);

// 認証が必要なAPIのハンドラに引数で渡すことで
// 認証のチェックを自動で行う
#[axum::async_trait]
impl<T> FromRequest<T> for UserId
where
    T: Send,
{
    // エラー時の戻り値の型
    type Rejection = (StatusCode, Json<serde_json::Value>);

    /// ログインが必要なリクエストに対して、Sessionの存在確認を行う
    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        // Extensinからメモリストアを取得
        let Extension(store) = Extension::<Store>::from_request(req).await.unwrap();

        // HTTPリクエストヘッダーからすべてのクッキーを取得
        let cookies = <TypedHeader<Cookie>>::from_request(req)
            .await
            .expect("Session store missing.");

        let session = match store.find_session(&cookies).await {
            Ok(session) => session,
            Err(_) => {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({"message": "Authentication required"})),
                ))
            }
        };

        // CookieにセッションIDが存在する場合は、Sessionからuser idを検索する
        let user_id = if let Some(user_id) = session.get(SESSION_USER_ID_KEY) {
            user_id
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "No user id found in session"})),
            ));
        };

        Ok(user_id)
    }
}
