use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::NaiveDate;

use crate::common::AppState;
use applications::stock::StockQueryCommand;

pub fn stock_controller(state: AppState) -> Router {
    Router::new()
        .route("/:stock_id", get(get_stocks))
        .with_state(state)
}

async fn get_stocks(
    state: State<AppState>,
    queries: Query<HashMap<String, String>>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let mut params = StockQueryCommand::new();
    params.stock_id = Some(stock_id);
    // クエリパラメータ取得
    params.start = if let Some(date) = queries.get("start") {
        let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { return "Err"};
        Some(date)
    } else {
        None
    };
    params.end = if let Some(date) = queries.get("end") {
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

    let result = state.stock_query_service().find(params).await.unwrap();

    "Ok"
}
