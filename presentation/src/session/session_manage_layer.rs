use std::sync::{Arc, RwLock};

use axum::{
    headers::{Cookie, HeaderValue},
    http::{self, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, TypedHeader,
};

use crate::{
    common::{internal_error, AppState},
    session::{SessionData, SessionFromRequest, SessionId},
};

pub type SharedSession = Arc<RwLock<SessionData>>;
const COOKIE_VALUE_KEY: &str = "Cookie Value";

/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<B>(
    Extension(utility): Extension<AppState>,
    TypedHeader(cookie_value): TypedHeader<Cookie>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let rejection = |_| internal_error().into_response();

    // RequestにCookieが設定されている場合
    let session = if let Some(cookie_value) = cookie_value.get(COOKIE_VALUE_KEY) {
        utility
            .session_service()
            .find_or_create(SessionId::new(cookie_value.to_string()))
            .await
            .map_err(rejection)?
    // Cookieが存在しない場合
    } else {
        utility
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

    // HandlerにSessionを受け渡す
    let session = Arc::new(RwLock::new(session));
    req.extensions_mut().insert(Arc::clone(&session));

    // 次のLayerを実行
    let mut response = next.run(req).await;
    // Cookie Headerに新しいSession Idを設定
    if is_created {
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

    // Sessionの変更を保存
    let mut session = session.read().unwrap().clone();
    if session.is_changed() {
        // SessionをCloneするとIdが削除されるため再度設定
        session.set_id(session_id);
        utility
            .session_service()
            .save(session)
            .await
            .map_err(rejection)?;
    }

    Ok(response)
}
