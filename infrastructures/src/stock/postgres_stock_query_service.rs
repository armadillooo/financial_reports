use chrono::NaiveDate;
use sqlx::{postgres::PgPool, Postgres, QueryBuilder};

use applications::stock::{StockData, StockQueryCommand, StockQueryResult, StockQueryService};

#[derive(Clone, Debug)]
pub struct PostgresStockQueryServiceImpl {
    connection: PgPool,
}

impl PostgresStockQueryServiceImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl StockQueryService for PostgresStockQueryServiceImpl {
    async fn find(&self, param: StockQueryCommand) -> StockQueryResult<Vec<StockData>> {
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("select * from stocks where id=");
        query.push_bind(&param.stock_id);

        if let Some(start) = &param.start {
            query.push(" and date>=");
            query.push_bind(start.format("%Y-%m-%d").to_string());
        }
        if let Some(end) = &param.end {
            query.push(" and date<=");
            query.push_bind(end.format("%Y-%m-%d").to_string());
        }

        let query = query.build_query_as();
        let result: Vec<StockModel> = query
            .fetch_all(&self.connection)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let result = result.into_iter().map(|s| s.into()).collect();
        Ok(result)
    }

    async fn find_latest(&self, stock_id: &str) -> StockQueryResult<StockData> {
        let result = sqlx::query_as!(
            StockModel,
            r#"select * from stocks where date=(select max(date) from stocks where stock_id=$1)"#,
            stock_id
        )
        .fetch_one(&self.connection)
        .await
        .map(|s| s.into())
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(result)
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct StockModel {
    stock_id: String,
    date: sqlx::types::time::Date,
    volume: i32,
    start_price: i32,
    end_price: i32,
    high_price: i32,
    low_price: i32,
}

impl Into<StockData> for StockModel {
    fn into(self) -> StockData {
        let date = NaiveDate::from_ymd_opt(
            self.date.year(),
            u8::from(self.date.month()) as u32,
            self.date.day() as u32,
        )
        .unwrap();

        StockData {
            stock_id: self.stock_id,
            date,
            volume: self.volume,
            start_price: self.start_price,
            end_price: self.end_price,
            high_price: self.high_price,
            low_price: self.low_price,
        }
    }
}
