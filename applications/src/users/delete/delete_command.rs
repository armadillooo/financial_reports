pub struct DeleteCommand {
    pub id: String,
    pub name: String,
}

impl DeleteCommand {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
