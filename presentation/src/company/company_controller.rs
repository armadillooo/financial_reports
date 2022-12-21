use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::common::AppState;
use applications::company::CompanyQueryCommand;

pub fn company_controller(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_companies))
        .with_state(state)
}

async fn get_companies(
    state: State<AppState>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mut params = CompanyQueryCommand::new();
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

    let result = state.company_query_service().find(params).await.unwrap();

    "Ok"
}
