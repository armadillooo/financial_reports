use axum::{middleware, Router};
use presentation::{
    common::{api_controllers, AppStateImpl},
    session::session_manage_layer,
};

/// アプリケーション初期化
pub fn init_app(state: AppStateImpl) -> Router {
    let app = api_controllers(state.clone()).layer(middleware::from_fn_with_state(
        state.clone(),
        session_manage_layer,
    ));

    app
}
