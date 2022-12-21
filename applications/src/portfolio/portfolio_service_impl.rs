use std::sync::Arc;

use anyhow::anyhow;
use futures::future::join_all;

use crate::{
    portfolio::{
        PortfoliApplicationResult, PortfolioData, PortfolioService, PortfolioUpdateCommand,
    },
    stock::StockQueryService,
};
use domain::{
    portfolio::{Portfolio, PortfolioReposotory},
    stock::StockId,
    users::{UserDomainService, UserId, UserRepository},
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct PortfolioServiceImpl<T, U, V>
where
    T: PortfolioReposotory,
    U: StockQueryService,
    V: UserRepository,
{
    portfolio_repository: Arc<T>,
    stock_query_service: U,
    user_service: UserDomainService<V>,
}

impl<T, U, V> PortfolioServiceImpl<T, U, V>
where
    T: PortfolioReposotory,
    U: StockQueryService,
    V: UserRepository,
{
    /// コンストラクタ
    pub fn new(
        portfolio_repository: &Arc<T>,
        stock_query_service: U,
        user_service: UserDomainService<V>,
    ) -> Self {
        Self {
            portfolio_repository: Arc::clone(portfolio_repository),
            stock_query_service,
            user_service,
        }
    }
}
#[async_trait::async_trait]
impl<T, U, V> PortfolioService for PortfolioServiceImpl<T, U, V>
where
    T: PortfolioReposotory + Send + Sync,
    U: StockQueryService + Send + Sync,
    V: UserRepository + Send + Sync,
{
    async fn get_all(&self, user_id: &str) -> PortfoliApplicationResult<Vec<PortfolioData>> {
        let user_id = UserId::new(user_id.to_string());
        let all_portfolio = self.portfolio_repository.find_all(&user_id).await?;

        // ポートフォリオを外部向けデータに変換
        let result = join_all(
            all_portfolio
                .into_iter()
                .map(|p| self.into_portfolio_data(p)),
        )
        .await;

        result.into_iter().collect()
    }

    async fn remove(&self, user_id: &str, stock_id: &str) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(user_id.to_string());
        let stock_id = StockId::new(stock_id.to_string());

        self.portfolio_repository
            .delete(&user_id, &stock_id)
            .await?;
        Ok(())
    }

    async fn update(
        &self,
        update_command: PortfolioUpdateCommand,
    ) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(update_command.user_id.to_string());
        let stock_id = StockId::new(update_command.stock_id.to_string());

        let mut portfolio = self
            .portfolio_repository
            .find(&user_id, &stock_id)
            .await?;

        if let Some(purchase) = update_command.purchase {
            portfolio.update_purchase(purchase);
        }
        if let Some(stock_count) = update_command.stock_count {
            portfolio.update_stock_count(stock_count);
        }
        if update_command.purchase.is_none() && update_command.stock_count.is_none() {
            return Ok(());
        }

        self.portfolio_repository.save(portfolio).await?;
        Ok(())
    }

    async fn add(&self, portfolio: PortfolioData) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(portfolio.user_id.to_string());
        
        self.user_service.exists(&user_id).await?;

        self.portfolio_repository.save(portfolio.into()).await?;
        Ok(())
    }
}

impl<T, U, V> PortfolioServiceImpl<T, U, V>
where
    T: PortfolioReposotory + Send + Sync,
    U: StockQueryService + Send + Sync,
    V: UserRepository + Send + Sync,
{
    async fn into_portfolio_data(
        &self,
        portfolio: Portfolio,
    ) -> PortfoliApplicationResult<PortfolioData> {
        let latest = self
            .stock_query_service
            .find_latest(&portfolio.stock_id)
            .await?;

        let portfolio_data = PortfolioData {
            stock_id: portfolio.stock_id.to_string(),
            user_id: portfolio.user_id.to_string(),
            stock_count: portfolio.stock_count,
            purchase: portfolio.purchase,
            market_price: latest.end_price,
            latest_date: latest.date,
        };

        Ok(portfolio_data)
    }
}
