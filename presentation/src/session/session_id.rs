use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct SessionId(String);

impl SessionId {
    /// コンストラクタ
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Deref for SessionId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
