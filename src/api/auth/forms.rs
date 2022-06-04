//! Http Request Form

use serde::Deserialize;

/// Login Form
#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

/// Signup Form
#[derive(Deserialize)]
pub struct Signup {
    pub email: String,
    pub username: String,
    pub password: String,
}
