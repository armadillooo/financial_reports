pub struct DeleteCommand {
    pub id: String,
}

impl DeleteCommand {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
