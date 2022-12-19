use std::sync::Arc;

use anyhow::anyhow;

use crate::{
    portfolio::{PortfolioData, PortfolioService, PortfolioUpdateCommand},
    stock::StockQueryService,
};
use domain::{
    portfolio::PortfolioReposotory,
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
        let result = self
            .portfolio_repository
            .find_all(&user_id)
            .await
            .map(|portfolios| {
                portfolios
                    .into_iter()
                    .map(|p| PortfolioData::from(p))
                    .collect()
            });

        result
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
