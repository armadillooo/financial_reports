use axum::extract::FromRequest;

use crate::session::SessionData;

#[async_trait::async_trait]
pub trait SessionService {
    type Data: SessionData;

    async fn find_or_create(
        &self,
        cookie_value: &str,
    ) -> anyhow::Result<SessionFromRequest<Self::Data>>;
    async fn save(&self, session: Self::Data) -> anyhow::Result<String>;
    async fn delete(&self, session: Self::Data) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub enum SessionFromRequest<T: SessionData> {
    Found(T),
    Created(CreatedSession<T>),
}

#[derive(Debug, Clone)]
pub struct CreatedSession<T: SessionData> {
    pub session: T,
    pub cookie: String,
}
