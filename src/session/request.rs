//! Http RequestからセッションIDを抽出する
use async_session::SessionStore;
use axum::{
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::session::{database::Store, SESSION_COOKIE_NAME, SESSION_USER_ID_KEY};

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
    type Rejection = (StatusCode, &'static str);

    /// ログインが必要なリクエストに対して、Sessionの存在確認を行う
    async fn from_request(req: &mut RequestParts<T>) -> Result<Self, Self::Rejection> {
        // Extensinからメモリストアを取得
        let Extension(store) = Extension::<Store>::from_request(req).await.unwrap();

        // HTTPリクエストヘッダーからすべてのクッキーを取得
        let cookies = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .expect("Session store missing.");

        // クッキーの中からセッションIDを取得する
        let session_cookie = cookies
            .as_ref()
            .and_then(|cookie| cookie.get(SESSION_COOKIE_NAME));

        // CookieがRequestにセットされていない
        if session_cookie.is_none() {
            return Err((StatusCode::FORBIDDEN, "Authentication required"));
        };

        // CookieにセッションIDが存在する場合は、Sessionからuser idを検索する
        let user_id = if let Some(session) = (*store.0)
            .load_session(session_cookie.unwrap().to_owned())
            .await
            .unwrap()
        {
            // user idがセッションに登録されている場合はユーザーIDを返す
            if let Some(user_id) = session.get::<UserId>(SESSION_USER_ID_KEY) {
                user_id
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No user id found in session",
                ));
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, "No session found for cookie"));
        };

        Ok(user_id)
    }
}
