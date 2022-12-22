use axum::{
    extract::State,
    headers::{Cookie, HeaderValue},
    http::{self, Request},
    middleware::Next,
    response::Response,
    TypedHeader,
};

use crate::{
    common::{ApiResult, AppState, AppStateImpl},
    session::{SessionId, SessionStatus},
};

const COOKIE_VALUE_KEY: &str = "Cookie Value";

/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<B>(
    state: State<AppStateImpl>,
    TypedHeader(cookie_value): TypedHeader<Cookie>,
    req: Request<B>,
    next: Next<B>,
) -> ApiResult<Response> {
    // RequestにCookieが設定されている場合
    let session_id = cookie_value
        .get(COOKIE_VALUE_KEY)
        .map(|cookie| SessionId::new(cookie.to_string()));

    let session_status = state.session_service().find_or_create(session_id).await?;

    // 次のLayerを実行
    let mut response = next.run(req).await;
    // Cookie Headerに新しいSession Idを設定
    if let SessionStatus::Created(session_id) = session_status {
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
