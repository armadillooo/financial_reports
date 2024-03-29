//! UserId Valueオブジェクト

use std::ops::Deref;
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct UserId(String);

impl UserId {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for UserId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String> for UserId {
    fn into(self) -> String {
        self.0
    }
}
