use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use async_redis_session::RedisSessionStore;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::{self, dotenv};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use applications::{
    favorite::FavoriteServiceImpl, portfolio::PortfolioServiceImpl, user::UserServiceImpl,
};
use domain::user::UserDomainService;
use financial_report::init_app;
use infrastructures::{
    auth::{OICDClient, OICDserviceImpl},
    company::PostgresCompanyQueryServiceImpl,
    favorite::PostgresFavoriteRepositoryImpl,
    portfolio::PostgresPortfolioRepositoryImpl,
    session::{SessionRepositoryImpl, SessionServiceImpl},
    stock::PostgresStockQueryServiceImpl,
    user::PostgresUserRepositoryImpl,
};
use presentation::common::AppStateImpl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            dotenvy::var("RUST_LOG").context("RUST_LOG does not exist")?,
        ))
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

    let state = setup_state().await?;

    let app = init_app(state);

    let addr = dotenvy::var("SOCKET_ADDRESS").context("SOCKET_ADDRESS does not exist")?;
    let addr = SocketAddr::from_str(&addr)?;

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn setup_state() -> anyhow::Result<AppStateImpl> {
    let pg_connection = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&dotenvy::var("DATABASE_URL").context("DATABASE_URL does not exist")?)
        .await?;

    let user_repository = Arc::new(PostgresUserRepositoryImpl::new(pg_connection.clone()));
    let user_service = UserServiceImpl::new(&user_repository);

    let redis =
        RedisSessionStore::new(dotenvy::var("SESSION_URL").context("SESSION_URL does not exist")?)?;
    let session_repository = Arc::new(SessionRepositoryImpl::new(redis));
    let session_service = SessionServiceImpl::new(&session_repository);

    let oicd_client = OICDClient::new(
        "https://accounts.google.com".to_string(),
        dotenvy::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID does not exist")?,
        dotenvy::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET does not exist")?,
        "https://127.0.0.1:3000/api/auth/redirect".to_string(),
    )
    .await?;
    let oicd_service = OICDserviceImpl::new(oicd_client);

    let stock_query_service = PostgresStockQueryServiceImpl::new(pg_connection.clone());

    let company_query_service = PostgresCompanyQueryServiceImpl::new(pg_connection.clone());

    let favorite_repository = Arc::new(PostgresFavoriteRepositoryImpl::new(pg_connection.clone()));
    let user_domain_service = UserDomainService::new(&user_repository);
    let favorite_service =
        FavoriteServiceImpl::new(&favorite_repository, user_domain_service.clone());

    let portfolio_repository = Arc::new(PostgresPortfolioRepositoryImpl::new(pg_connection));
    let portfolio_service = PortfolioServiceImpl::new(
        &portfolio_repository,
        stock_query_service.clone(),
        user_domain_service,
    );

    let state = AppStateImpl::new(
        Arc::new(user_service),
        Arc::new(session_service),
        Arc::new(oicd_service),
        Arc::new(stock_query_service),
        Arc::new(company_query_service),
        Arc::new(favorite_service),
        Arc::new(portfolio_service),
    );

    Ok(state)
}
