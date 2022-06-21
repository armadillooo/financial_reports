//! Userドメインモデル

pub mod user_id;
pub mod user_name;
pub mod user_repository;
pub mod user_service;

use user_id::UserId;
use user_name::UserName;

/// Userドメインモデル
pub struct User {
    /// Unique id
    id: UserId,
    /// Unique name
    name: UserName,
}
