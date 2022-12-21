use crate::{auth::OICDService, session::SessionService};
use applications::{
    company::CompanyQueryService, favorite::FavoriteService, portfolio::PortfolioService,
    stock::StockQueryService, users::UserService,
};

/// ハンドラ間で共有されるオブジェクト
pub trait Utility {
    type UserApplicationServiceState: UserService;
    type SessionServiceState: SessionService;
    type OICDServiceState: OICDService;
    type StockQueryServiceState: StockQueryService;
    type CompanyQueryServiceState: CompanyQueryService;
    type FavoriteServiceState: FavoriteService;
    type PortfolioServiceState: PortfolioService;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState;
    fn session_service(&self) -> &Self::SessionServiceState;
    fn oicd_service(&self) -> &Self::OICDServiceState;
    fn stock_query_service(&self) -> &Self::StockQueryServiceState;
    fn company_query_service(&self) -> &Self::CompanyQueryServiceState;
    fn favorite_service(&self) -> &Self::FavoriteServiceState;
    fn portfolio_service(&self) -> &Self::PortfolioServiceState;
}
