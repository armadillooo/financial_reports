mod api;
mod session;

use std::{net::SocketAddr, str::FromStr};

use dotenvy::{self, dotenv};

use api::api_routes;

#[tokio::main]
async fn main() {
    // .envファイルを読み込み
    dotenv().ok();
    // Default Logger初期化
    tracing_subscriber::fmt::init();

    let app = api_routes();

    let addr = dotenvy::var("SOCKET_ADDRESS").unwrap();
    let addr = SocketAddr::from_str(&addr).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
