use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// ユーザー情報取得
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
}
