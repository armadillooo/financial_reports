use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionId(String);

impl SessionId {
    /// コンストラクタ
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

impl Into<String> for SessionId {
    fn into(self) -> String {
        self.0
    }
}

impl Deref for SessionId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for SessionId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
