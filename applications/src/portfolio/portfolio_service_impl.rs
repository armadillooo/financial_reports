use std::sync::Arc;

use futures::future::join_all;

use crate::{
    portfolio::{
        PortfoliApplicationResult, PortfolioApplicationError, PortfolioData, PortfolioService,
        PortfolioUpdateCommand,
    },
    stock::StockQueryService,
};
use domain::{
    portfolio::{Portfolio, PortfolioReposotory},
    stock::StockId,
    user::{UserDomainService, UserId, UserRepository},
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
    user_domain_service: UserDomainService<V>,
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
        user_domain_service: UserDomainService<V>,
    ) -> Self {
        Self {
            portfolio_repository: Arc::clone(portfolio_repository),
            stock_query_service,
            user_domain_service,
        }
    }
}
#[async_trait::async_trait]
impl<T, U, V> PortfolioService for PortfolioServiceImpl<T, U, V>
where
    T: PortfolioReposotory + std::fmt::Debug + Send + Sync,
    U: StockQueryService + std::fmt::Debug + Send + Sync,
    V: UserRepository + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(skip(self), err, ret)]
    async fn get_all(&self, user_id: &str) -> PortfoliApplicationResult<Vec<PortfolioData>> {
        let user_id = UserId::new(user_id.into());
        self.user_domain_service.exists(&user_id).await?;

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

    #[tracing::instrument(skip(self), err)]
    async fn remove(&self, user_id: &str, stock_id: &str) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(user_id.into());
        self.user_domain_service.exists(&user_id).await?;

        let stock_id = StockId::new(stock_id.to_string());

        self.portfolio_repository
            .delete(&user_id, &stock_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self), err)]
    async fn update(
        &self,
        update_command: PortfolioUpdateCommand,
    ) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(update_command.user_id.into());
        self.user_domain_service.exists(&user_id).await?;

        let stock_id = StockId::new(update_command.stock_id.into());

        let mut portfolio = self
            .portfolio_repository
            .find(&user_id, &stock_id)
            .await?
            .ok_or(PortfolioApplicationError::PortfolioNotFound(
                stock_id.into(),
            ))?;

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

    #[tracing::instrument(skip(self), err)]
    async fn add(&self, portfolio: PortfolioData) -> PortfoliApplicationResult<()> {
        let user_id = UserId::new(portfolio.clone().user_id.into());
        self.user_domain_service.exists(&user_id).await?;

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::anyhow;

    use crate::{
        portfolio::{
            InmemoryPortfolioRepositoryImpl, PortfolioApplicationError, PortfolioData,
            PortfolioService, PortfolioServiceImpl, PortfolioUpdateCommand,
        },
        stock::{InmemoryStockQueryServiceImpl, StockData},
        user::InmemoryUserRepositoryImpl,
    };
    use domain::{
        stock::StockId,
        user::{User, UserDomainService, UserEmail, UserId, UserName, UserRepository},
    };

    const USER_ID: &str = "sample user";
    const STOCK_ID: &str = "sample stock";

    async fn setup() -> impl PortfolioService {
        let mut stock_query_service = InmemoryStockQueryServiceImpl::new();
        let mut sample_stock = StockData::new();
        sample_stock.stock_id = StockId::new(STOCK_ID.to_string());
        stock_query_service.stocks.push(sample_stock);

        let user_repository = Arc::new(InmemoryUserRepositoryImpl::new());
        let sample_user = User::new(
            UserId::new(USER_ID.to_string()),
            UserName::new("".to_string()),
            UserEmail::new("".to_string()),
        );
        user_repository.save(sample_user).await.unwrap();

        let user_domain_service = UserDomainService::new(&user_repository);

        let portfolio_repository = Arc::new(InmemoryPortfolioRepositoryImpl::new());
        let portfolio_service = PortfolioServiceImpl::new(
            &portfolio_repository,
            stock_query_service.clone(),
            user_domain_service,
        );

        portfolio_service
    }

    #[tokio::test]
    async fn add_portfolio_success() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData::new(USER_ID.to_string(), STOCK_ID.to_string());
        service.add(portfolio).await?;

        let result = service.get_all(USER_ID).await?;
        assert!(result[0].user_id == USER_ID);
        assert!(result.len() == 1);

        Ok(())
    }

    #[tokio::test]
    async fn add_notexist_user_portfolio_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData::new("not registed user".to_string(), STOCK_ID.to_string());

        let Err(PortfolioApplicationError::UserNotFound(_)) = service.add(portfolio.clone()).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn get_all_notexist_user_portfolio_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData::new("not registed user".to_string(), STOCK_ID.to_string());

        let Err(PortfolioApplicationError::UserNotFound(_)) = service.get_all(&portfolio.user_id).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn remove_portfolio_success() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData::new(USER_ID.to_string(), STOCK_ID.to_string());
        service.add(portfolio.clone()).await?;

        let result = service.get_all(USER_ID).await?;
        assert!(result[0].user_id == USER_ID);
        assert!(result.len() == 1);

        service
            .remove(&portfolio.user_id, &portfolio.stock_id)
            .await?;
        let result = service.get_all(USER_ID).await?;
        assert!(result.len() == 0);

        Ok(())
    }

    #[tokio::test]
    async fn remove_notexist_user_portfolio_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData::new("not registed user".to_string(), STOCK_ID.to_string());

        let Err(PortfolioApplicationError::UserNotFound(_)) = service.remove(&portfolio.user_id, &portfolio.stock_id).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn update_portfolio_success() -> anyhow::Result<()> {
        let service = setup().await;
        let purchase = 12;
        let stock_count = 444;
        let portfolio = PortfolioData {
            user_id: USER_ID.to_string(),
            stock_id: STOCK_ID.to_string(),
            ..Default::default()
        };
        let command = PortfolioUpdateCommand::new(
            USER_ID.to_string(),
            STOCK_ID.to_string(),
            Some(purchase),
            Some(stock_count),
        );

        service.add(portfolio.clone()).await?;
        service.update(command).await?;
        let result = service
            .get_all(&portfolio.user_id)
            .await?
            .pop()
            .ok_or(anyhow!("portfolio not found"))?;

        assert!(result.stock_count == stock_count);
        assert!(result.purchase == purchase);

        Ok(())
    }

    #[tokio::test]
    async fn update_notexist_user_portfolio_return_err() -> anyhow::Result<()> {
        let service = setup().await;
        let command = PortfolioUpdateCommand::new(
            "not registed user".to_string(),
            STOCK_ID.to_string(),
            None,
            None,
        );

        let Err(PortfolioApplicationError::UserNotFound(_)) = service.update(command).await else {
            return Err(anyhow!("unexpected add favorite result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn update_portfolio_with_noparameter_nochange() -> anyhow::Result<()> {
        let service = setup().await;
        let portfolio = PortfolioData {
            user_id: USER_ID.to_string(),
            stock_id: STOCK_ID.to_string(),
            stock_count: 43,
            purchase: 98,
            ..Default::default()
        };
        let command = PortfolioUpdateCommand::new(
            USER_ID.to_string(),
            STOCK_ID.to_string(),
            None,
            None,
        );

        service.add(portfolio.clone()).await?;
        service.update(command).await?;
        let result = service
            .get_all(&portfolio.user_id)
            .await?
            .pop()
            .ok_or(anyhow!("portfolio not found"))?;

        assert!(result == portfolio);

        Ok(())
    }
}
