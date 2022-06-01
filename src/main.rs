mod api;
mod session;

use std::{net::SocketAddr, str::FromStr};

use axum::Extension;
use dotenvy::{self, dotenv};

use api::app_config;
use api::database::Db;
use session::database::Store;

#[tokio::main]
async fn main() {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::fmt::init();

    let pool = Db::new().await;
    let store = Store::new();
    let app = app_config().layer(Extension(pool)).layer(Extension(store));

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
