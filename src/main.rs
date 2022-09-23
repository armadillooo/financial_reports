use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use async_session::MemoryStore;
use axum::{middleware, Extension};
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::{self, dotenv};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};
use presentation::{
    auth::{OICDClient, OICDserviceImpl},
    common::UtilityImpl,
    controllers,
    session::{session_manage_layer, SessionRepositoryImpl, SessionServiceImpl},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var(
            "RUST_LOG",
        )?))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("self_signed_certs");
    tracing::debug!("tls config directory = {:?}", base_path);

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await?;

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let user_service = UserApplicationServiceImpl::new(&user_repository);
    let session_repository = Arc::new(SessionRepositoryImpl::new(MemoryStore::new()));
    let session_service = SessionServiceImpl::new(&session_repository);
    let oicd_client = OICDClient::new(
        "https://accounts.google.com".to_string(),
        "525690818902-l0urmj6r09omclbguobeq6ef1iqr561k.apps.googleusercontent.com".to_string(),
        "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".to_string(),
        "https://127.0.0.1:3000/api/auth/redirect".to_string(),
    )
    .await?;
    let oicd_service = OICDserviceImpl::new(oicd_client);

    let state = UtilityImpl::new(user_service, session_service, oicd_service);

    let app = controllers()
        .layer(middleware::from_fn(session_manage_layer))
        .layer(Extension(state));

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
