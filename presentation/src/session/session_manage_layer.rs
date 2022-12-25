use axum::{
    extract::{State},
    headers::{Cookie, HeaderValue},
    http::{self, Request, Extensions},
    middleware::Next,
    response::Response,
    TypedHeader,
};

use crate::{
    common::{ApiResult, AppState, AppStateImpl},
    session::{SessionId, SessionStatus},
};

const COOKIE_VALUE_KEY: &str = "Cookie Value";

#[tracing::instrument(skip(state, req, next, cookie_value), err)]
/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<B: std::fmt::Debug>(
    state: State<AppStateImpl>,
    TypedHeader(cookie_value): TypedHeader<Cookie>,
    mut req: Request<B>,
    next: Next<B>,
) -> ApiResult<Response> {
    // RequestにCookieが設定されている場合
    let session_id = cookie_value
        .get(COOKIE_VALUE_KEY)
        .map(|cookie| SessionId::new(cookie.to_string()));

    let session_status = state.session_service().find_or_create(session_id).await?;
    let is_session_created = if let SessionStatus::Created(_) = session_status {
        true
    } else {
        false
    };
    let session_id: SessionId = session_status.into();
    // SessionIdをハンドラから参照できるようにする
    let mut extension = Extensions::new();
    extension.insert(session_id.clone());
    req.extensions_mut().extend(extension);

    // 次のLayerを実行
    let mut response = next.run(req).await;
    // Cookie Headerに新しいSession Idを設定
    if is_session_created {
        response.headers_mut().insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(&format!(
                "{}={}; path=/",
                COOKIE_VALUE_KEY,
                session_id.to_string()
            ))
            .expect("Cookie format is invalid"),
        );
    }

    Ok(response)
}
