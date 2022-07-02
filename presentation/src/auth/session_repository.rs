use super::session_data::SessionData;

pub trait SessionRepository<T>
where
    T: SessionData,
{
    fn save(&self, session_id: &str) -> anyhow::Result<()>;
    fn find(&self, session: T) -> anyhow::Result<T>;
    fn delete(&self, session: T) -> anyhow::Result<()>;
}
