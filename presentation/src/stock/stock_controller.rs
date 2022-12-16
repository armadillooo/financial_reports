use std::collections::HashMap;

use chrono::NaiveDate;

use applications::stock::{
    CompanyQueryParameters, CompanyQueryService, StockQueryParameters, StockQueryService,
};
use axum::{
    extract::{Extension, Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::common::{Utility, UtilityImpl};

pub fn stock_cotroller() -> Router {
    Router::new()
        .route("/stocks/:stock_id", get(get_stocks))
        .route("/companies", get(get_companies))
}

async fn get_stocks(
    Extension(utility): Extension<UtilityImpl>,
    Query(queries): Query<HashMap<String, String>>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let mut params = StockQueryParameters::new();
    params.stock_id = Some(stock_id);
    // クエリパラメータ取得
    params.date_from = if let Some(date) = queries.get("date_from") {
        let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { return "Err"};
        Some(date)
    } else {
        None
    };
    params.date_to = if let Some(date) = queries.get("date_to") {
        let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { return "Err"};
        Some(date)
    } else {
        None
    };
    params.page = if let Some(page) = queries.get("page") {
        let Ok(page) = page.parse() else { return "Err"};
        Some(page)
    } else {
        None
    };
    params.size = if let Some(size) = queries.get("size") {
        let Ok(page) = size.parse() else { return "Err"};
        Some(page)
    } else {
        None
    };

    let result = utility.stock_query_service().find(params).await.unwrap();

    "Ok"
}

async fn get_companies(
    Extension(utility): Extension<UtilityImpl>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut params = CompanyQueryParameters::new();
    // クエリパラメータ取得
    params.name = queries.get("name").map(|s| s.to_string());
    params.stock_id = queries.get("stock_id").map(|s| s.to_string());
    params.sector = queries.get("sector").map(|s| s.to_string());
    params.industry = queries.get("industry").map(|s| s.to_string());
    params.page = if let Some(page) = queries.get("page") {
        let Ok(page) = page.parse() else { return "Err"};
        Some(page)
    } else {
        None
    };
    params.size = if let Some(size) = queries.get("size") {
        let Ok(page) = size.parse() else { return "Err"};
        Some(page)
    } else {
        None
    };

    let result = utility.company_query_service().find(params).await.unwrap();

    "Ok"
}
