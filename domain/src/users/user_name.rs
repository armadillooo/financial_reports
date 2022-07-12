//! UserName Valueオブジェクト

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UserName {
    value: String,
}

impl UserName {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}