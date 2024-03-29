use std::collections::HashMap;

use axum::{
    extract::{Extension, Query, State},
    headers::{HeaderMap, HeaderValue},
    http,
    response::{IntoResponse, Response},
    routing::get,
    Router, Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::OICDData,
    common::{ApiResult, AppState, AppStateImpl},
    session::{SessionError, SessionId, SessionItem},
    user::{LoginUserId, UserResponse},
};
use applications::user::{UserApplicationError, UserData};

use super::OICDError;

pub fn auth_controller(state: AppStateImpl) -> Router {
    Router::new()
        .route("/signin", get(signin_redirect_google))
        .route("/login", get(login_redirect_google))
        .route("/logout", get(logout))
        .route("/redirect", get(auth_verify_google))
        .with_state(state)
}

/// ユーザー新規作成
#[tracing::instrument(skip(state), err)]
async fn signin_redirect_google(
    Extension(session_id): Extension<SessionId>,
    state: State<AppStateImpl>,
) -> ApiResult<Response> {
    state
        .session_service()
        .insert_item(session_id.clone(), SessionItem::AuthType(AuthType::Singin))
        .await?;

    oicd_redirect(session_id, &state).await
}

/// ログイン
#[tracing::instrument(skip(state), err)]
async fn login_redirect_google(
    Extension(session_id): Extension<SessionId>,
    state: State<AppStateImpl>,
) -> ApiResult<Response> {
    state
        .session_service()
        .insert_item(session_id.clone(), SessionItem::AuthType(AuthType::Login))
        .await?;

    oicd_redirect(session_id, &state).await
}

/// 認証結果検証
#[tracing::instrument(skip(state, params), err)]
async fn auth_verify_google(
    Extension(session_id): Extension<SessionId>,
    state: State<AppStateImpl>,
    params: Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let key = SessionItem::AuthType(AuthType::Singin);
    let Some(SessionItem::AuthType(auth_type)) = state.session_service().find_item(session_id.clone(), &key).await?  else {
        return Err(SessionError::ItemNotFound(key.key()).into());
    };

    // 認証に成功した場合はユーザー情報を取得
    let auth_user = oicd_verify(session_id.clone(), &state, params).await?;

    match auth_type {
        AuthType::Login => {
            // ユーザー未登録
            if let None = state.user_application_service().get(&auth_user.id).await? {
                return Err(UserApplicationError::UserNotExist(auth_user.id.clone()).into());
            }
        }
        AuthType::Singin => {
            // ユーザーが既に存在するため新規追加不可
            if let Some(_) = state.user_application_service().get(&auth_user.id).await? {
                return Err(UserApplicationError::UserAlreadyExist(auth_user.id.clone()).into());
            } else {
                state
                    .user_application_service()
                    .save(auth_user.clone())
                    .await?;
            }
        }
    };

    // セッションにユーザー情報を追加
    let item = SessionItem::LoginUserId(LoginUserId::new(auth_user.id.clone()));
    state
        .session_service()
        .insert_item(session_id, item)
        .await?;

    let result = UserResponse::from(auth_user);

    Ok(Json(result).into_response())
}

/// ログアウト
async fn logout(
    Extension(session_id): Extension<SessionId>,
    user_id: LoginUserId,
    state: State<AppStateImpl>,
) -> ApiResult<Response> {
    let key = SessionItem::LoginUserId(user_id);
    state
        .session_service()
        .remove_item(session_id, &key)
        .await?;

    let result = serde_json::json!({ "message": "succeed in logout"});
    
    Ok(Json(result).into_response())
}

async fn oicd_redirect(session_id: SessionId, state: &AppStateImpl) -> ApiResult<Response> {
    let auth_info = state.oicd_service().redirect().await;
    let redirect_url = auth_info.auth_url.clone();

    let item = SessionItem::AuthInfo(auth_info);
    state
        .session_service()
        .insert_item(session_id, item)
        .await?;

    let mut header = HeaderMap::new();
    header.insert(
        http::header::LOCATION,
        HeaderValue::from_str(&redirect_url.to_string()).unwrap(),
    );

    let res = (http::StatusCode::FOUND, header).into_response();
    Ok(res)
}

async fn oicd_verify(
    session_id: SessionId,
    state: &AppStateImpl,
    params: Query<HashMap<String, String>>,
) -> ApiResult<UserData> {
    let key = SessionItem::AuthInfo(OICDData::new());
    let Some(SessionItem::AuthInfo(oicd_info)) = state.session_service().find_item(session_id.clone(), &key).await? else {
        return Err(SessionError::ItemNotFound(key.key()).into());
    };

    let Some(code) = params.get("code") else {
            return Err(OICDError::ParameterRequired { name: "code" }.into());
        };
    let Some(oicd_state) = params.get("state") else {
        return Err(OICDError::ParameterRequired { name: "state" }.into());
    };

    let user = state
        .oicd_service()
        .verify(oicd_info, code.to_owned(), oicd_state.to_owned())
        .await?;

    // 不要なデータを削除
    let key = SessionItem::AuthInfo(OICDData::new());
    state
        .session_service()
        .remove_item(session_id.clone(), &key)
        .await?;

    // ユーザーIDをセッションに登録
    let auth_user_id = SessionItem::LoginUserId(LoginUserId::new(user.id.clone()));
    state
        .session_service()
        .insert_item(session_id, auth_user_id)
        .await?;

    Ok(user)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthType {
    Login,
    Singin,
}
