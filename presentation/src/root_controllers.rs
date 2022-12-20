use axum::Router;

use crate::{
    auth::auth_controller, company::company_controller, stock::stock_controller,
    user::user_controller,
};

pub fn root_controllers() -> Router {
    let api_routes = Router::new()
        .route("/auth", auth_controller())
        .route("/stocks", stock_controller())
        .route("/companies", company_controller())
        .route("/users", user_controller());

    let root = Router::new().nest("/api", api_routes);

    root
}
