use std::sync::Arc;

use anyhow::anyhow;
use futures::future::join_all;

use crate::{
    portfolio::{PortfolioData, PortfolioService, PortfolioUpdateCommand},
    stock::StockQueryService,
};
use domain::{
    portfolio::{Portfolio, PortfolioReposotory},
    stock::StockId,
    users::{UserId, UserRepository, UserService},
};

pub struct PortfolioServieImpl<T, U, V>
where
    T: PortfolioReposotory,
    U: StockQueryService,
    V: UserRepository,
{
    portfolio_repository: Arc<T>,
    stock_query_service: Arc<U>,
    user_service: UserService<V>,
}

#[async_trait::async_trait]
impl<T, U, V> PortfolioService for PortfolioServieImpl<T, U, V>
where
    T: PortfolioReposotory + Send + Sync,
    U: StockQueryService + Send + Sync,
    V: UserRepository + Send + Sync,
{
    async fn get_all(&self, user_id: &str) -> anyhow::Result<Vec<PortfolioData>> {
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

    async fn remove(&self, user_id: &str, stock_id: &str) -> anyhow::Result<()> {
        let user_id = UserId::new(user_id.to_string());
        let stock_id = StockId::new(stock_id.to_string());

        self.portfolio_repository.delete(&user_id, &stock_id).await
    }

    async fn update(&self, update_command: PortfolioUpdateCommand) -> anyhow::Result<()> {
        let user_id = UserId::new(update_command.user_id.to_string());
        let stock_id = StockId::new(update_command.stock_id.to_string());

        let mut portfolio = self
            .portfolio_repository
            .find(&user_id, &stock_id)
            .await?
            .ok_or(anyhow!("Portfolio not found"))?;

        if let Some(purchase) = update_command.purchase {
            portfolio.update_purchase(purchase);
        }
        if let Some(stock_count) = update_command.stock_count {
            portfolio.update_stock_count(stock_count);
        }
        if update_command.purchase.is_none() && update_command.stock_count.is_none() {
            return Ok(());
        }

        self.portfolio_repository.save(portfolio).await
    }

    async fn add(&self, portfolio: PortfolioData) -> anyhow::Result<()> {
        let user_id = UserId::new(portfolio.user_id.to_string());
        if self.user_service.exists(&user_id).await == false {
            return Err(anyhow!("user not exists"));
        };

        self.portfolio_repository.save(portfolio.into()).await
    }
}

impl<T, U, V> PortfolioServieImpl<T, U, V>
where
    T: PortfolioReposotory + Send + Sync,
    U: StockQueryService + Send + Sync,
    V: UserRepository + Send + Sync,
{
    async fn into_portfolio_data(&self, portfolio: Portfolio) -> anyhow::Result<PortfolioData> {
        let latest = self
            .stock_query_service
            .find_latest(&portfolio.stock_id)
            .await?
            .ok_or(anyhow!("Latest stock data not found"))?;

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
