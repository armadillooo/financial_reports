mod api;
mod session;

use std::net::SocketAddr;

use api::api_routes;

#[tokio::main]
async fn main() {
    let app = api_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
