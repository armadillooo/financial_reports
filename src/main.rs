mod api;
mod session;

use std::str::FromStr;
use std::sync::Arc;
use std::{net::SocketAddr, ops::Deref};

use async_session::MemoryStore;
use axum::{middleware, response::IntoResponse, routing::get, Extension, Router};
use dotenvy::{self, dotenv};

use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};
use infrastructures::{
    common::StateImpl,
    session::{SessionRepositoryImpl, SessionServiceImpl},
};
use presentation::session::{SessionId, SessionService};
use presentation::{
    common::State,
    session::{session_manage_layer, ItemKey},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    type StateType = StateImpl;

    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::fmt::init();

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service = UserApplicationServiceImpl::new(&user_repository);
    let session_repository = Arc::new(SessionRepositoryImpl::new(MemoryStore::new()));
    let session_service = SessionServiceImpl::new(&session_repository);

    let state = StateImpl::new(user_service, session_service);

    let app = Router::new()
        .route("/", get(handler::<StateType>))
        .layer(Extension(state))
        .layer(middleware::from_fn(session_manage_layer::<StateType, _>));

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler<T: State>(
    state: Extension<T>,
    session_id: Extension<SessionId>,
) -> impl IntoResponse {
    let mut session = state
        .session_service()
        .find_or_create(&session_id)
        .await
        .unwrap();
    let key = ItemKey::<i32>::new("counter".to_string());
}
