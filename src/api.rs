mod auth;
mod reports;
mod users;

use auth::routes::auth_api_routes;
use axum::Router;
use reports::routes::reports_api_routes;
use users::routes::users_api_routes;

/// 各apiのルーティングを"/api"以下に集約する
pub fn api_routes() -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth_api_routes())
        .nest("/reports", reports_api_routes())
        .nest("/users", users_api_routes());

    Router::new().nest("/api", api_routes)
}