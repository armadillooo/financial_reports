use std::collections::HashMap;

use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::common::{Utility, UtilityImpl};
use applications::company::{CompanyQueryParameters, CompanyQueryService};

pub fn company_controller() -> Router {
    Router::new().route("/", get(get_companies))
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
