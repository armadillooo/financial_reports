use async_session::SessionStore;
use axum::{
    extract::Json,
    headers::Cookie,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Router, TypedHeader,
};
use serde_json::json;

use super::{
    forms::{Login, Signup},
    models::User,
};
use crate::{
    api::database::Db,
    session::{database::Store, request::UserId},
};

/// auth apiのエンドポイント
pub fn auth_api_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

/// ログイン・セッション新規作成
async fn login(
    Extension(store): Extension<Store>,
    Extension(pool): Extension<Db>,
    cookies: TypedHeader<Cookie>,
    form: Json<Login>,
) -> Response {
    //ユーザー情報を取得
    let user = if let Ok(record) = sqlx::query_as!(
        User,
        "select * from users where email = $1 AND password = $2;",
        form.email,
        form.password
    )
    .fetch_one(&(*pool.0))
    .await
    {
        record
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Email or password mismatch"})),
        )
            .into_response();
    };

    // Sessionが既に存在するかをチェック
    if let Ok(session) = store.find_session(&cookies).await {
        if let Some(user_id) = store.find_user_id(&session) {
            if user_id.0 == user.id {
                return (
                StatusCode::OK,
                Json(
                    json!({"message": "You are already logged in", "username": user.username, "email": user.email}),
                ),
            ).into_response();
            // Sessionのuser idが送られたものと異なる
            } else {
                store.destroy_session(session).await.unwrap();
            }
        // Sessionにuser idが存在しない
        } else {
            store.destroy_session(session).await.unwrap();
        }
    };

    // Sessionを新規作成
    let header = if let Ok(header) = store.start_session(UserId(user.id)).await {
        header
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Email or password mismatch"})),
        )
            .into_response();
    };

    (StatusCode::OK, header).into_response()
}

/// ログアウト・セッション削除
///
/// UserIDは認証確認用のため変数は使用しない
async fn logout(
    Extension(store): Extension<Store>,
    cookie: TypedHeader<Cookie>,
    _: UserId,
) -> Response {
    let session = if let Ok(session) = store.find_session(&cookie).await {
        session
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "There is no session"})),
        )
            .into_response();
    };

    let header = if let Ok(header) = store.reset_session(session).await {
        header
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Session has already deleted"})),
        )
            .into_response();
    };

    (StatusCode::OK, header).into_response()
}

/// ユーザー新規作成
async fn signup(Extension(pool): Extension<Db>, form: Json<Signup>) -> Response {
    let user = if let Ok(record) = sqlx::query_as!(
        User,
        "insert into users (email, username, password) VALUES ($1, $2, $3) RETURNING id, email, username, password;",
        form.email,
        form.username,
        form.password
    )
    .fetch_one(&(*pool.0))
    .await
    {
        record
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Email and username must be unique"})),
        )
            .into_response();
    };

    (
        StatusCode::CREATED,
        Json(json!({"username": user.username, "email": user.email})),
    )
        .into_response()
}
