use sqlx::postgres::PgPool;

use domain::{
    portfolio::{Portfolio, PortfolioDomainResult, PortfolioReposotory},
    stock::StockId,
    user::UserId,
};

#[derive(Clone, Debug)]
pub struct PostgresPortfolioRepositoryImpl {
    connection: PgPool,
}

impl PostgresPortfolioRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl PortfolioReposotory for PostgresPortfolioRepositoryImpl {
    async fn save(&self, portfolio: Portfolio) -> PortfolioDomainResult<()> {
        sqlx::query!(
            r#"
            insert into portfolio values ($1, $2, $3, $4)
            on conflict (user_id, stock_id)
            do update set (stock_count, purchase) = ($3, $4)
            "#,
            portfolio.user_id.as_str(),
            portfolio.stock_id.as_str(),
            portfolio.stock_count,
            portfolio.purchase,
        )
        .execute(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId, stock_id: &StockId) -> PortfolioDomainResult<()> {
        sqlx::query!(
            r#"delete from portfolio where user_id=$1 and stock_id=$2"#,
            user_id.as_str(),
            stock_id.as_str(),
        )
        .execute(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    async fn find_all(&self, user_id: &UserId) -> PortfolioDomainResult<Vec<Portfolio>> {
        let result = sqlx::query_as!(
            PortfolioModel,
            r#"select * from portfolio where user_id=$1"#,
            user_id.as_str()
        )
        .fetch_all(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        let result = result.into_iter().map(|p| p.into()).collect();
        Ok(result)
    }

    async fn find(
        &self,
        user_id: &UserId,
        stock_id: &StockId,
    ) -> PortfolioDomainResult<Option<Portfolio>> {
        let result = sqlx::query_as!(
            PortfolioModel,
            r#"select * from portfolio where user_id=$1 and stock_id=$2"#,
            user_id.as_str(),
            stock_id.as_str()
        )
        .fetch_optional(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .map(|p| p.into());

        Ok(result)
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct PortfolioModel {
    user_id: String,
    stock_id: String,
    stock_count: i32,
    purchase: i32,
}

impl Into<Portfolio> for PortfolioModel {
    fn into(self) -> Portfolio {
        let user_id = UserId::new(self.user_id);
        let stock_id = StockId::new(self.stock_id);
        Portfolio {
            user_id,
            stock_id,
            stock_count: self.stock_count,
            purchase: self.purchase,
        }
    }
}
