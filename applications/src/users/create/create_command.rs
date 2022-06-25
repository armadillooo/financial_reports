pub struct CreateCommand {
    pub id: String,
    pub name: String,
}

impl CreateCommand {
    /// コンストラクタ
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
