pub trait SessionData {
    fn set_user_id(&mut self, user_id: String) -> anyhow::Result<()>;
    fn user_id(&self) -> anyhow::Result<Option<String>>;
    fn session_id(&self) -> String;
}
