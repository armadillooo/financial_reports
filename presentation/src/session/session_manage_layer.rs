use axum::{http::Request, middleware::Next, response::IntoResponse};

/// Sessionが新規作成された場合にCookiにSession IDを自動で追加する
pub async fn session_manage_layer<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    next.run(req).await;

    "hello from middleware"
}
