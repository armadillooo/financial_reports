use std::collections::HashMap;

use axum::{
    extract::{Extension, Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::NaiveDate;

use crate::common::{Utility, UtilityImpl};
use applications::stock::{StockQueryCommand, StockQueryService};

pub fn stock_controller() -> Router {
    Router::new().route("/:stock_id", get(get_stocks))
}

async fn get_stocks(
    Extension(utility): Extension<UtilityImpl>,
    Query(queries): Query<HashMap<String, String>>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let mut params = StockQueryCommand::new();
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
