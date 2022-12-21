use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    headers::{HeaderMap, HeaderValue},
    http,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{OICDData, OICDService},
    common::{ErrorResponse, JsonBuilder, Utility, UtilityImpl},
    session::{ItemKey, SharedSession},
    user::{LoginedUserId, USER_ID},
};
use applications::users::{UserData, UserService};

const AUTH_TYPE: ItemKey<AuthenticationType> = ItemKey::new("auth type");
const OICD_INFO: ItemKey<OICDData> = ItemKey::new("oicd info");

pub fn auth_controller() -> Router {
    Router::new()
        .route("/signin", get(signin_redirect_google))
        .route("/login", get(login_redirect_google))
        .route("/logout", get(logout))
        .route("/redirect", get(auth_verify_google))
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
async fn auth_verify_google(
    session: Extension<SharedSession>,
    utility: Extension<UtilityImpl>,
    params: Query<HashMap<String, String>>,
) -> Response {
    let auth_type = if let Some(item) = session.read().unwrap().item(&AUTH_TYPE) {
        item
    } else {
        return (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            JsonBuilder::new()
                .add(ErrorResponse {
                    message: "Internal server error occured",
                })
                .build(),
        )
            .into_response();
    };

    let auth_user = if let Ok(user) = oicd_verify(&utility, &session, params).await {
        user
    } else {
        return (
            http::StatusCode::BAD_REQUEST,
            JsonBuilder::new()
                .add(ErrorResponse {
                    message: "Authentication failed",
                })
                .build(),
        )
            .into_response();
    };
    match auth_type {
        AuthenticationType::Login => {
            // ログイン成功
            if let Some(user) = utility
                .user_application_service()
                .get(&auth_user.id)
                .await
                .unwrap()
            {
                (http::StatusCode::OK, JsonBuilder::new().add(user).build()).into_response()
            // ログイン失敗
            } else {
                (
                    http::StatusCode::BAD_REQUEST,
                    JsonBuilder::new()
                        .add(ErrorResponse {
                            message: "User registration required",
                        })
                        .build(),
                )
                    .into_response()
            }
        }
        AuthenticationType::Singin => {
            // ユーザーが既に存在するため新規追加不可
            if let Some(_) = utility
                .user_application_service()
                .get(&auth_user.id)
                .await
                .unwrap()
            {
                (
                    http::StatusCode::BAD_REQUEST,
                    JsonBuilder::new()
                        .add(ErrorResponse {
                            message: "User is already exist",
                        })
                        .build(),
                )
                    .into_response()
            // ユーザー新規作成可能
            } else {
                utility
                    .user_application_service()
                    .save(auth_user.clone())
                    .await
                    .unwrap();
                (
                    http::StatusCode::OK,
                    JsonBuilder::new().add(auth_user).build(),
                )
                    .into_response()
            }
        }
    }
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
