//! UserName Valueオブジェクト

use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UserName(String);

impl UserName {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for UserName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
