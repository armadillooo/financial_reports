use async_session::MemoryStore;

use crate::{
    auth::OICDserviceImpl,
    common::Utility,
    session::{SessionRepositoryImpl, SessionServiceImpl},
};
use applications::users::{InMemoryUserRepository, UserApplicationServiceImpl};

type UserApplicationServiceType = UserApplicationServiceImpl<InMemoryUserRepository>;
type SessionServiceType = SessionServiceImpl<SessionRepositoryImpl<MemoryStore>>;
type OICDServiceType = OICDserviceImpl;

#[derive(Debug, Clone)]
pub struct UtilityImpl {
    user_application_service: UserApplicationServiceType,
    session_service: SessionServiceType,
    oicd_service: OICDServiceType,
}

impl UtilityImpl {
    pub fn new(
        user_application_service: UserApplicationServiceType,
        session_service: SessionServiceType,
        oicd_service: OICDServiceType,
    ) -> Self {
        Self {
            user_application_service,
            session_service,
            oicd_service,
        }
    }
}

impl Utility for UtilityImpl {
    type UserApplicationServiceState = UserApplicationServiceType;
    type SessionServiceState = SessionServiceType;
    type OICDServiceState = OICDserviceImpl;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState {
        &self.user_application_service
    }

    fn session_service(&self) -> &Self::SessionServiceState {
        &self.session_service
    }

    fn oicd_service(&self) -> &Self::OICDServiceState {
        &self.oicd_service
    }
}
