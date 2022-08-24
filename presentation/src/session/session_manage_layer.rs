use std::sync::{Arc, RwLock};

use axum::{
    extract::{FromRequest, RequestParts},
    headers::{Cookie, HeaderValue},
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension, TypedHeader,
};

use crate::{
    common::{ApiError, JsonBuilder, Rejection, Utility},
    session::{SessionData, SessionFromRequest, SessionId, SessionService},
};

pub type SharedSession = Arc<RwLock<SessionData>>;

const COOKIE_VALUE_KEY: &str = "Cookie Value";

/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<T, B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Rejection>
where
    T: Utility + Send + Sync + Clone + 'static,
    B: Send + Sync,
{
    // エラー時の戻り値
    let rejection = |_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonBuilder::new()
                .add(ApiError {
                    message: "Internal server error occured",
                })
                .build(),
        )
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
            .and_then(|cookies| {
                cookies
                    .get(COOKIE_VALUE_KEY)
                    .map(|cookie| cookie.to_owned())
            }) {
        state
            .session_service()
            .find_or_create(SessionId::new(cookie_value.to_string()))
            .await
            .map_err(rejection)?
    // Cookieが存在しない場合
    } else {
        state
            .session_service()
            .create()
            .await
            .map(|session| SessionFromRequest::Created(session))
            .map_err(rejection)?
    };

    let (session, is_created, session_id) = match session {
        SessionFromRequest::Created(session) => (session.inner, true, session.id),
        SessionFromRequest::Found(session) => (session.inner, false, session.id),
    };

    let mut req = request_parts
        .try_into_request()
        .expect("Request body extracted");

    // HandlerにSessionを受け渡す
    let session = Arc::new(RwLock::new(session));
    req.extensions_mut().insert(Arc::clone(&session));

    // 次のLayerを実行
    let mut response = next.run(req).await;
    // Cookie Headerに新しいSession Idを設定
    if is_created {
        response.headers_mut().insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("{}={}", COOKIE_VALUE_KEY, session_id.to_string()))
                .expect("Cookie format is invalid"),
        );
    }

    // Sessionの変更を保存
    let mut session = session.read().unwrap().clone();
    if session.is_changed() {
        // SessionをCloneするとIdが削除されるため再度設定
        session.set_id(session_id);
        state
            .session_service()
            .save(session)
            .await
            .map_err(rejection)?;
    }

    Ok(response)
}
