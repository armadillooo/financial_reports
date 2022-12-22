use std::sync::Arc;

use crate::{auth::OICDService, session::SessionService};
use applications::{
    company::CompanyQueryService, favorite::FavoriteService, portfolio::PortfolioService,
    stock::StockQueryService, users::UserService,
};

#[async_trait::async_trait]
pub trait AppState {
    fn user_application_service(&self) -> &Arc<dyn UserService + Send + Sync>;
    fn session_service(&self) -> &Arc<dyn SessionService + Send + Sync>;
    fn oicd_service(&self) -> &Arc<dyn OICDService + Send + Sync>;
    fn stock_query_service(&self) -> &Arc<dyn StockQueryService + Send + Sync>;
    fn company_query_service(&self) -> &Arc<dyn CompanyQueryService + Send + Sync>;
    fn favorite_service(&self) -> &Arc<dyn FavoriteService + Send + Sync>;
    fn portfolio_service(&self) -> &Arc<dyn PortfolioService + Send + Sync>;
}
