//! UserId Valueオブジェクト
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UserId {
    value: String,
}

impl UserId {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
