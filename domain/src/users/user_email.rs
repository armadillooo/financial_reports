use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Debug)]
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
