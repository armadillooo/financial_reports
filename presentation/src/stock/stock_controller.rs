use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::NaiveDate;

use crate::common::{ApiResponse, ApiResult, AppState, AppStateImpl};
use applications::stock::{StockQueryCommand, StockQueryError};

use super::StockResponse;

pub fn stock_controller(state: AppStateImpl) -> Router {
    Router::new()
        .route("/:stock_id", get(get_stocks))
        .with_state(state)
}

#[tracing::instrument(skip(state, queries), err)]
async fn get_stocks(
    state: State<AppStateImpl>,
    queries: Query<HashMap<String, String>>,
    Path(stock_id): Path<String>,
) -> ApiResult<Response> {
    let mut params = StockQueryCommand::new();

    params.stock_id = Some(stock_id);
    // クエリパラメータ取得
    params.start = if let Some(date) = queries.get("start") {
        let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { return Err(StockQueryError::InvalidParameter { name: "end", value: date.clone() }.into())};
        Some(date)
    } else {
        None
    };
    params.end = if let Some(date) = queries.get("end") {
        let Ok(date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") else { return Err(StockQueryError::InvalidParameter { name: "end", value: date.clone() }.into())};
        Some(date)
    } else {
        None
    };
    params.page = if let Some(page) = queries.get("page") {
        let Ok(page) = page.parse() else { return Err(StockQueryError::InvalidParameter { name: "page", value: page.clone() }.into())};
        Some(page)
    } else {
        None
    };
    params.size = if let Some(size) = queries.get("size") {
        let Ok(page) = size.parse() else { return Err(StockQueryError::InvalidParameter { name: "size", value: size.clone() }.into())};
        Some(page)
    } else {
        None
    };

    let result = state.stock_query_service().find(params).await?;

    let res: ApiResponse<Vec<StockResponse>> =
        ApiResponse::new(result.into_iter().map(|s| StockResponse::from(s)).collect());
    Ok(res.into_response())
}
