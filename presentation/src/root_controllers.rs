use axum::Router;

use crate::{
    auth::auth_controller, company::company_controller, stock::stock_controller,
    user::user_controller,
};

pub fn root_controllers() -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth_controller())
        .nest("/stocks", stock_controller())
        .nest("/companies", company_controller())
        .nest("/users", user_controller());

    let root = Router::new().nest("/api", api_routes);

    root
}