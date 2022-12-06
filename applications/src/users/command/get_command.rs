pub struct GetCommand {
    pub id: String,
}

impl GetCommand {
    /// コンストラクタ
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
