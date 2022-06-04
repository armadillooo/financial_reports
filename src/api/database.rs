//! DBへ接続する
use std::ops::Deref;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

/// DB接続用インターフェース
///
/// axumのExtensionで共有するためCloneトレイトを実装
#[derive(Clone)]
pub struct Db(pub(crate) Arc<PgPool>);

impl Db {
    /// コネクションプール生成
    ///
    /// 非同期タスク間で共有するためArc<>に内包させる
    pub async fn new() -> Db {
        let database_url = dotenvy::var("DATABASE_URL").unwrap();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap();

        Db(Arc::new(pool))
    }
}

impl Deref for Db {
    type Target = Arc<PgPool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
