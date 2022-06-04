use axum::{
    extract::Form,
    headers::Cookie,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Extension, Router, TypedHeader,
};

use super::forms::{Login, Signup};
use crate::{
    api::database::Db,
    session::{database::Store, request::UserId},
};

/// auth apiのエンドポイント
pub fn auth_api_routes() -> Router {
    Router::new()
        .route("/singup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

/// ログイン・セッション新規作成
async fn login(
    Extension(store): Extension<Store>,
    Extension(pool): Extension<Db>,
    form: Form<Login>,
) -> Response {
    //ユーザー情報を取得
    let user = if let Ok(record) = sqlx::query!(
        "select * from users where email = $1 AND password = $2;",
        form.email,
        form.password
    )
    .fetch_one(&(*pool.0))
    .await
    {
        record
    } else {
        return (StatusCode::BAD_REQUEST, "Email or password mismatch").into_response();
    };

    // Sessionを新規作成
    let header = if let Ok(header) = store.regist_userid(UserId(user.id)).await {
        header
    } else {
        return (StatusCode::BAD_REQUEST, "Email or password mismatch").into_response();
    };

    (StatusCode::OK, header).into_response()
}

/// ログアウト・セッション削除
///
/// user_idは認証確認用のため、変数は使用しない
async fn logout(
    Extension(store): Extension<Store>,
    cookie: TypedHeader<Cookie>,
    _: UserId,
) -> Response {
    let session = if let Ok(session) = store.find_session(&cookie).await {
        session
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "There is no session").into_response();
    };

    let header = if let Ok(header) = store.delete_session(session).await {
        header
    } else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Session has already deleted",
        )
            .into_response();
    };

    (StatusCode::OK, header).into_response()
}

/// ユーザー新規作成
async fn signup(Extension(pool): Extension<Db>, form: Form<Signup>) -> Response {
    if let Err(_) = sqlx::query!(
        "INSERT INTO Users (email, username, password) VALUES ($1, $2, $3) RETURNING id;",
        form.email,
        form.username,
        form.password
    )
    .fetch_one(&(*pool.0))
    .await
    {
        return (StatusCode::BAD_REQUEST, "Email and username must be unique").into_response();
    };

    StatusCode::OK.into_response()
}
