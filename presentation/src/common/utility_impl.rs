use async_session::MemoryStore;

use crate::{
    auth::OICDserviceImpl,
    common::Utility,
    session::{SessionRepositoryImpl, SessionServiceImpl},
};
use applications::{
    stock::{InmemoryCompanyQueryServiceImpl, InmemoryStockQueryServiceImpl},
    users::{InMemoryUserRepositoryImpl, UserApplicationServiceImpl},
};

type UserApplicationServiceType = UserApplicationServiceImpl<InMemoryUserRepositoryImpl>;
type SessionServiceType = SessionServiceImpl<SessionRepositoryImpl<MemoryStore>>;
type OICDServiceType = OICDserviceImpl;
type StockQueryServiceType = InmemoryStockQueryServiceImpl;
type CompanyQueryServiceType = InmemoryCompanyQueryServiceImpl;

#[derive(Debug, Clone)]
pub struct UtilityImpl {
    user_application_service: UserApplicationServiceType,
    session_service: SessionServiceType,
    oicd_service: OICDServiceType,
    stock_query_service: StockQueryServiceType,
    company_query_service: CompanyQueryServiceType,
}

impl UtilityImpl {
    pub fn new(
        user_application_service: UserApplicationServiceType,
        session_service: SessionServiceType,
        oicd_service: OICDServiceType,
        stock_query_service: StockQueryServiceType,
        company_query_service: CompanyQueryServiceType,
    ) -> Self {
        Self {
            user_application_service,
            session_service,
            oicd_service,
            stock_query_service,
            company_query_service,
        }
    }
}

impl Utility for UtilityImpl {
    type UserApplicationServiceState = UserApplicationServiceType;
    type SessionServiceState = SessionServiceType;
    type OICDServiceState = OICDServiceType;
    type StockQueryServiceState = StockQueryServiceType;
    type CompanyQueryServiceState = CompanyQueryServiceType;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState {
        &self.user_application_service
    }

    fn session_service(&self) -> &Self::SessionServiceState {
        &self.session_service
    }

    fn oicd_service(&self) -> &Self::OICDServiceState {
        &self.oicd_service
    }

    fn stock_query_service(&self) -> &Self::StockQueryServiceState {
        &self.stock_query_service
    }

    fn company_query_service(&self) -> &Self::CompanyQueryServiceState {
        &self.company_query_service
    }
}
