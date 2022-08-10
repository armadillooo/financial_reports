use axum::{response::Response, routing::get, Router};

pub fn auth_controller() -> Router {
    let auth_root = Router::new()
        .nest("/google", get(authorize))
        .nest("/authorized", get(login_authorized));

    Router::new().nest("/auth", auth_root)
}

async fn authorize() -> Response {
    unimplemented!()
}

async fn login_authorized() -> Response {
    unimplemented!()
}
