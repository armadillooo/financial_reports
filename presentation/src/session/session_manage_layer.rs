use axum::{
    extract::State,
    headers::{Cookie, HeaderValue},
    http::{self, Extensions, Request},
    middleware::Next,
    response::Response,
    TypedHeader,
};

use crate::{
    common::{ApiResult, AppState, AppStateImpl},
    session::SessionId,
};

const COOKIE_VALUE_KEY: &str = "Cookie Value";

#[tracing::instrument(skip(state, req, next, cookie_value), err)]
/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<B: std::fmt::Debug>(
    state: State<AppStateImpl>,
    cookie_value: Option<TypedHeader<Cookie>>,
    mut req: Request<B>,
    next: Next<B>,
) -> ApiResult<Response> {
    // RequestにCookieが設定されている場合
    let session_id = if let Some(cookie_value) = cookie_value {
        cookie_value
            .get(COOKIE_VALUE_KEY)
            .map(|cookie| SessionId::new(cookie.to_string()))
    } else {
        tracing::info!("cookie header was not sent");
        None
    };
    let session_status = state.session_service().find_or_create(session_id).await?;

    // SessionIdをハンドラから参照できるようにする
    let session_id: SessionId = session_status.into();
    let mut extension = Extensions::new();
    extension.insert(session_id.clone());
    req.extensions_mut().extend(extension);

    // 次のLayerを実行
    let mut response = next.run(req).await;

    // レスポンスにSet-Cookieヘッダーを追加
    let session_id: &str = &session_id;
    response.headers_mut().insert(
        http::header::SET_COOKIE,
        HeaderValue::from_str(&format!(
            "{COOKIE_VALUE_KEY}={session_id}; Secure; SameSite=None; HttpOnly; Path=/;"
        ))
        .expect("set-cookie format is invalid"),
    );

    Ok(response)
}
