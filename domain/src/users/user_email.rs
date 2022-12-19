use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserEmail(String);

impl UserEmail {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for UserEmail {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
