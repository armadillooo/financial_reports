use axum::{extract::Extension, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};

use crate::{
    auth::OICDData,
    session::{ItemKey, SharedSession},
};

const AUTH_TYPE: ItemKey<AuthenticationType> = ItemKey::new("auth type");
const OICD_VERIFY_INFO: ItemKey<OICDData> = ItemKey::new("oicd info");

pub fn auth_controller() -> Router {
    let auth_root = Router::new()
        .nest("/signin", get(signin_redirect_google))
        .nest("/login", get(login_redirect_google))
        .nest("/logout", get(logout))
        .nest("/google", get(auth_finished_google));

    Router::new().nest("/auth", auth_root)
}

/// ユーザー新規作成
async fn signin_redirect_google(Extension(session): Extension<SharedSession>) -> impl IntoResponse {
    session
        .write()
        .unwrap()
        .insert_item(&AUTH_TYPE, AuthenticationType::Singin)
        .unwrap();

    ""
}

/// ログイン
async fn login_redirect_google(Extension(session): Extension<SharedSession>) -> impl IntoResponse {
    session
        .write()
        .unwrap()
        .insert_item(&AUTH_TYPE, AuthenticationType::Login)
        .unwrap();

    ""
}

/// 認証結果検証
async fn auth_finished_google(Extension(session): Extension<SharedSession>) -> impl IntoResponse {
    let auth_type = if let Some(item) = session.read().unwrap().item(&AUTH_TYPE) {
        item
    } else {
        return "";
    };

    match auth_type {
        AuthenticationType::Login => "",
        AuthenticationType::Singin => "",
    }
}

/// ログアウト
async fn logout() -> impl IntoResponse {
    unimplemented!()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum AuthenticationType {
    Login,
    Singin,
}
