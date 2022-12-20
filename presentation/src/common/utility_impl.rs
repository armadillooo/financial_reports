use async_session::MemoryStore;

use crate::{
    auth::OICDserviceImpl,
    common::Utility,
    session::{SessionRepositoryImpl, SessionServiceImpl},
};
use applications::{
    company::InmemoryCompanyQueryServiceImpl,
    favorite::{FavoriteServiceImpl, InmemoryFavoriteRepositoryImpl},
    portfolio::{InmemoryPortfolioRepositoryImpl, PortfolioServiceImpl},
    stock::InmemoryStockQueryServiceImpl,
    users::{InMemoryUserRepositoryImpl, UserApplicationServiceImpl},
};

type UserApplicationServiceType = UserApplicationServiceImpl<InMemoryUserRepositoryImpl>;
type SessionServiceType = SessionServiceImpl<SessionRepositoryImpl<MemoryStore>>;
type OICDServiceType = OICDserviceImpl;
type StockQueryServiceType = InmemoryStockQueryServiceImpl;
type CompanyQueryServiceType = InmemoryCompanyQueryServiceImpl;
type FavoriteServiceType =
    FavoriteServiceImpl<InmemoryFavoriteRepositoryImpl, InMemoryUserRepositoryImpl>;
type PortfolioServiceType = PortfolioServiceImpl<
    InmemoryPortfolioRepositoryImpl,
    InmemoryStockQueryServiceImpl,
    InMemoryUserRepositoryImpl,
>;

#[derive(Debug, Clone)]
pub struct UtilityImpl {
    user_application_service: UserApplicationServiceType,
    session_service: SessionServiceType,
    oicd_service: OICDServiceType,
    stock_query_service: StockQueryServiceType,
    company_query_service: CompanyQueryServiceType,
    favorite_service: FavoriteServiceType,
    portfolio_service: PortfolioServiceType,
}

impl UtilityImpl {
    pub fn new(
        user_application_service: UserApplicationServiceType,
        session_service: SessionServiceType,
        oicd_service: OICDServiceType,
        stock_query_service: StockQueryServiceType,
        company_query_service: CompanyQueryServiceType,
        favorite_service: FavoriteServiceType,
        portfolio_service: PortfolioServiceType,
    ) -> Self {
        Self {
            user_application_service,
            session_service,
            oicd_service,
            stock_query_service,
            company_query_service,
            favorite_service,
            portfolio_service,
        }
    }
}

impl Utility for UtilityImpl {
    type UserApplicationServiceState = UserApplicationServiceType;
    type SessionServiceState = SessionServiceType;
    type OICDServiceState = OICDServiceType;
    type StockQueryServiceState = StockQueryServiceType;
    type CompanyQueryServiceState = CompanyQueryServiceType;
    type FavoriteServiceState = FavoriteServiceType;
    type PortfolioServiceState = PortfolioServiceType;

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

    fn favorite_service(&self) -> &Self::FavoriteServiceState {
        &self.favorite_service
    }

    fn portfolio_service(&self) -> &Self::PortfolioServiceState {
        &self.portfolio_service
    }
}
