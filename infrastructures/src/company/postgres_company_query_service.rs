use sqlx::{postgres::PgPool, Postgres, QueryBuilder};

use applications::company::{
    CompanyData, CompanyQueryCommand, CompanyQueryResult, CompanyQueryService,
};

#[derive(Clone, Debug)]
pub struct PostgresCompanyQueryServiceImpl {
    connection: PgPool,
}

impl PostgresCompanyQueryServiceImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl CompanyQueryService for PostgresCompanyQueryServiceImpl {
    async fn find(&self, param: CompanyQueryCommand) -> CompanyQueryResult<Vec<CompanyData>> {
        let result = sqlx::query_as!(
            CompanyData,
            r#"select * from companies where stock_id=$1"#,
            param.stock_id
        )
        .fetch_all(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        let result = result.into_iter().map(|s| s.into()).collect();
        Ok(result)
    }

    async fn find_by_id(&self, stock_id: &str) -> CompanyQueryResult<CompanyData> {
        let result = sqlx::query_as!(
            CompanyData,
            r#"select * from companies where stock_id=$1"#,
            stock_id,
        )
        .fetch_one(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(result)
    }

    async fn find_list(&self, stock_id_list: Vec<String>) -> CompanyQueryResult<Vec<CompanyData>> {
        if stock_id_list.is_empty() {
            return Ok(vec![]);
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("select * from companies where stock_id in (");

        // パラメータをクエリに追加
        let mut query_params = query_builder.separated(", ");
        for id in stock_id_list.iter() {
            query_params.push_bind(id);
        }
        query_params.push_unseparated(")");

        // query生成
        let query = query_builder.build_query_as();
        let result: Vec<CompanyModel> = query
            .fetch_all(&self.connection)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let result = result.into_iter().map(|c| c.into()).collect();
        Ok(result)
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct CompanyModel {
    name: String,
    stock_id: String,
    sector: String,
    industry: String,
}

impl Into<CompanyData> for CompanyModel {
    fn into(self) -> CompanyData {
        CompanyData {
            name: self.name,
            stock_id: self.stock_id,
            sector: self.sector,
            industry: self.industry,
        }
    }
}
