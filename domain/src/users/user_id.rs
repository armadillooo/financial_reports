//! UserId Valueオブジェクト
use ulid::Ulid;

#[derive(PartialEq, Eq)]
pub struct UserId {
    value: String,
}

impl UserId {
    /// コンストラクタ
    pub fn new() -> Self {
        Self { value: Ulid::new().to_string() }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
