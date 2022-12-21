use axum::Router;

use crate::{
    auth::auth_controller, common::AppState, company::company_controller, stock::stock_controller,
    user::user_controller,
};

pub fn root_controllers(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/auth", auth_controller(state.clone()))
        .nest("/stocks", stock_controller(state.clone()))
        .nest("/companies", company_controller(state.clone()))
        .nest("/users", user_controller(state));

    let root = Router::new().nest("/api", api_routes);

    root
}
