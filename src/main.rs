use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use async_session::MemoryStore;
use axum::{Extension, Router};
use dotenvy::{self, dotenv};

use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};
use infrastructures::{
    common::State,
    session::{SessionRepositoryImpl, SessionServiceImpl},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::fmt::init();

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service = UserApplicationServiceImpl::new(&user_repository);

    let session_repository = Arc::new(SessionRepositoryImpl::new(MemoryStore::new()));
    let session_service = SessionServiceImpl::new(&session_repository);

    let state = State::new(user_service, session_service);

    let app = Router::new().layer(Extension(state));

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
