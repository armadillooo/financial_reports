mod api;
mod session;

use std::{net::SocketAddr, str::FromStr};

use dotenvy::{self, dotenv};

use api::{app_config, database::connect_pool};

#[tokio::main]
async fn main() {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::fmt::init();

    let pool = connect_pool().await;

    let app = app_config();

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
