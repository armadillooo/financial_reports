use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    headers::{HeaderMap, HeaderValue},
    http,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{OICDData, OICDService},
    common::{Utility, UtilityImpl},
    session::{ItemKey, SharedSession},
    user::{LoginedUserId, USER_ID},
};
use applications::users::{UserData, UserApplicationService, GetCommand};

const AUTH_TYPE: ItemKey<AuthenticationType> = ItemKey::new("auth type");
const OICD_INFO: ItemKey<OICDData> = ItemKey::new("oicd info");

pub fn auth_controller() -> Router {
    let auth_root = Router::new()
        .nest("/signin", get(signin_redirect_google))
        .nest("/login", get(login_redirect_google))
        .nest("/logout", get(logout))
        .nest("/google", get(auth_finished_google));

    Router::new().nest("/auth", auth_root)
}

/// ユーザー新規作成
async fn signin_redirect_google(
    Extension(session): Extension<SharedSession>,
    Extension(utility): Extension<UtilityImpl>,
) -> impl IntoResponse {
    session
        .write()
        .unwrap()
        .insert_item(&AUTH_TYPE, AuthenticationType::Singin)
        .unwrap();

    oicd_redirect(&utility, &session).await
}

/// ログイン
async fn login_redirect_google(
    session: Extension<SharedSession>,
    utility: Extension<UtilityImpl>,
) -> impl IntoResponse {
    session
        .write()
        .unwrap()
        .insert_item(&AUTH_TYPE, AuthenticationType::Login)
        .unwrap();

    oicd_redirect(&utility, &session).await
}

/// 認証結果検証
async fn auth_finished_google(
    session: Extension<SharedSession>,
    utility: Extension<UtilityImpl>,
    params: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let auth_type = if let Some(item) = session.read().unwrap().item(&AUTH_TYPE) {
        item
    } else {
        return "";
    };

    let user = oicd_verify(&utility, &session, params).await.unwrap();

    match auth_type {
        AuthenticationType::Login => {
            let command = GetCommand::new(user.id);
            if let Ok(user) = utility.user_application_service().get(command).await {
                
            }
        },
        AuthenticationType::Singin => {},
    };

    ""
}

/// ログアウト
async fn logout() -> impl IntoResponse {
    unimplemented!()
}

async fn oicd_redirect(utility: &UtilityImpl, session: &SharedSession) -> impl IntoResponse {
    let verify_info = utility.oicd_service().redirect().await;
    let redirect_url = verify_info.auth_url.clone();
    session
        .write()
        .unwrap()
        .insert_item(&OICD_INFO, verify_info)
        .unwrap();

    let mut header = HeaderMap::new();
    header.insert(
        http::header::LOCATION,
        HeaderValue::from_str(&redirect_url.to_string()).unwrap(),
    );

    (http::StatusCode::FOUND, header)
}

async fn oicd_verify(
    utility: &UtilityImpl,
    session: &SharedSession,
    params: Query<HashMap<String, String>>,
) -> anyhow::Result<UserData> {
    let oicd_info = session
        .read()
        .unwrap()
        .item(&OICD_INFO)
        .expect("There is no verify info in the session");

    let code = params.get("code").expect("query param 'code' is not set");
    let state = params.get("state").expect("query param 'state' is not set");

    utility
        .oicd_service()
        .verify(oicd_info, code.to_owned(), state.to_owned())
        .await
        .map(|user| {
            // 不要なデータをSessionから削除
            session.write().unwrap().remove_item(&OICD_INFO);
            session
                .write()
                .unwrap()
                .insert_item(&USER_ID, LoginedUserId::new(user.id.clone()))
                .unwrap();

            user
        })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum AuthenticationType {
    Login,
    Singin,
}
