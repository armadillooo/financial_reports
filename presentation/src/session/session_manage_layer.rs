use axum::{
    extract::{FromRequest, Json, RequestParts},
    headers::{Cookie, HeaderValue},
    http::{self, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, TypedHeader,
};

use crate::common::{ApiError, JsonBuilder, State};

use super::{SessionFromRequest, SessionId, SessionService, SessionWithId};

const COOKIE_VALUE_KEY: &str = "Cookie value";

/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<T, B>(req: Request<B>, next: Next<B>) -> Response
where
    T: State + Send + Sync + Clone + 'static,
    B: Send + Sync,
{
    // エラー時の戻り値
    let rejection = || {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                JsonBuilder::new()
                    .add(ApiError {
                        message: "Session was not established",
                    })
                    .build(),
            ),
        )
            .into_response()
    };

    let mut request_parts = RequestParts::new(req);
    let state = Extension::<T>::from_request(&mut request_parts)
        .await
        .expect("State extension was not set");

    // RequestにCookieが設定されている場合
    let session = if let Some(cookie_value) =
        <TypedHeader<Cookie>>::from_request(&mut request_parts)
            .await
            .ok()
            .and_then(|cookies| cookies.get(COOKIE_VALUE_KEY).map(|key| key.to_owned()))
    {
        let session_id = SessionId::new(cookie_value);
        if let Ok(session) = state.session_service().find_or_create(&session_id).await {
            session
        } else {
            return rejection();
        }
    // Cookieが存在しない場合
    } else {
        let session = if let Ok(session) = state.session_service().create().await {
            session
        } else {
            return rejection();
        };
        SessionFromRequest::Refreshed(session)
    };

    let (session_id, is_created) = match session {
        SessionFromRequest::Refreshed(session) => (session.id, true),
        SessionFromRequest::Found(session) => (session.id, false),
    };

    let mut req = request_parts
        .try_into_request()
        .expect("Request body extracted");
    // HandlerにSession IDを渡す
    req.extensions_mut().insert(session_id.clone());

    // 次のLayerを実行
    let mut response = next.run(req).await;
    // Cookie Headerに新しいSession Idを設定
    if is_created {
        response.headers_mut().insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("{}={}", COOKIE_VALUE_KEY, *session_id))
                .expect("Cookie format is invalid"),
        );
    }

    response
}
