use async_session::MemoryStore;

use crate::session::{SessionRepositoryImpl, SessionServiceImpl};
use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};
use presentation::common::State;

type UserApplicationServiceType = UserApplicationServiceImpl<InMemoryUserRepository>;
type SessionServiceType = SessionServiceImpl<SessionRepositoryImpl<MemoryStore>>;

#[derive(Debug, Clone)]
pub struct StateImpl {
    user_application_service: UserApplicationServiceType,
    session_service: SessionServiceType,
}

impl StateImpl {
    pub fn new(
        user_application_service: UserApplicationServiceType,
        session_service: SessionServiceType,
    ) -> Self {
        Self {
            user_application_service,
            session_service,
        }
    }
}

impl State for StateImpl {
    type UserApplicationServiceState = UserApplicationServiceType;
    type SessionServiceState = SessionServiceType;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState {
        &self.user_application_service
    }

    fn session_service(&self) -> &Self::SessionServiceState {
        &self.session_service
    }
}
