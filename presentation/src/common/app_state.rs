use std::sync::Arc;

use crate::{auth::OICDService, session::SessionService};
use applications::{
    company::CompanyQueryService, favorite::FavoriteService, portfolio::PortfolioService,
    stock::StockQueryService, users::UserService,
};

#[derive(Clone)]
pub struct AppState {
    user_application_service: Arc<dyn UserService + Send + Sync>,
    session_service: Arc<dyn SessionService + Send + Sync>,
    oicd_service: Arc<dyn OICDService + Send + Sync>,
    stock_query_service: Arc<dyn StockQueryService + Send + Sync>,
    company_query_service: Arc<dyn CompanyQueryService + Send + Sync>,
    favorite_service: Arc<dyn FavoriteService + Send + Sync>,
    portfolio_service: Arc<dyn PortfolioService + Send + Sync>,
}

impl AppState {
    pub fn new(
        user_application_service: Arc<dyn UserService + Send + Sync>,
        session_service: Arc<dyn SessionService + Send + Sync>,
        oicd_service: Arc<dyn OICDService + Send + Sync>,
        stock_query_service: Arc<dyn StockQueryService + Send + Sync>,
        company_query_service: Arc<dyn CompanyQueryService + Send + Sync>,
        favorite_service: Arc<dyn FavoriteService + Send + Sync>,
        portfolio_service: Arc<dyn PortfolioService + Send + Sync>,
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

    pub fn user_application_service(&self) -> &Arc<dyn UserService + Send + Sync> {
        &self.user_application_service
    }

    pub fn session_service(&self) -> &Arc<dyn SessionService + Send + Sync> {
        &self.session_service
    }

    pub fn oicd_service(&self) -> &Arc<dyn OICDService + Send + Sync> {
        &self.oicd_service
    }

    pub fn stock_query_service(&self) -> &Arc<dyn StockQueryService + Send + Sync> {
        &self.stock_query_service
    }

    pub fn company_query_service(&self) -> &Arc<dyn CompanyQueryService + Send + Sync> {
        &self.company_query_service
    }

    pub fn favorite_service(&self) -> &Arc<dyn FavoriteService + Send + Sync> {
        &self.favorite_service
    }

    pub fn portfolio_service(&self) -> &Arc<dyn PortfolioService + Send + Sync> {
        &self.portfolio_service
    }
}
