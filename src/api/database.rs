//! DBへ接続する
use sqlx::postgres::{PgPool, PgPoolOptions};

/// コネクションプール初期化
pub async fn connect_pool() -> PgPool {
    let database_url = dotenvy::var("DATABASE_URL").unwrap();

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap()
}
