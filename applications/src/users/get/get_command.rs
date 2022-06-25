pub struct GetCommand {
    pub id: String,
}

impl GetCommand {
    /// コンストラクタ
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
