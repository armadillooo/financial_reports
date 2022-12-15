use crate::{auth::OICDService, session::SessionService};
use applications::{
    stock::{CompanyQueryService, StockQueryService},
    users::UserApplicationService,
};

/// ハンドラ間で共有されるオブジェクト
pub trait Utility {
    type UserApplicationServiceState: UserApplicationService;
    type SessionServiceState: SessionService;
    type OICDServiceState: OICDService;
    type StockQueryServiceState: StockQueryService;
    type CompanyQueryServiceState: CompanyQueryService;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState;
    fn session_service(&self) -> &Self::SessionServiceState;
    fn oicd_service(&self) -> &Self::OICDServiceState;
    fn stock_query_service(&self) -> &Self::StockQueryServiceState;
    fn company_query_service(&self) -> &Self::CompanyQueryServiceState;
}
