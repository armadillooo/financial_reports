use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct StockId(String);

impl StockId {
    /// コンストラクタ
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Deref for StockId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
