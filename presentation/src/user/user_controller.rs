use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{
    common::{ApiResult, AppState, AppStateImpl},
    user::LoginUserId,
};
use applications::{
    favorite::FavoriteData,
    portfolio::{PortfolioApplicationError, PortfolioData, PortfolioUpdateCommand},
    user::UserApplicationError,
};

use crate::user::{FavoriteResponse, PortfolioResponse, UserResponse};

pub fn user_controller(state: AppStateImpl) -> Router {
    let user_route = Router::new()
        .route("/", get(get_user))
        .route("/favorites", get(get_favorites))
        .route(
            "/favorites/:stock_id",
            post(insert_favorite).delete(delete_favorite),
        )
        .route("/portfolio", get(get_portfolio))
        .route(
            "/portfolio/:stock_id",
            post(insert_portfolio)
                .patch(update_portfolio)
                .delete(delete_portfolio),
        )
        .with_state(state);

    Router::new().nest("/me", user_route)
}

#[tracing::instrument(skip(state), err)]
async fn get_user(state: State<AppStateImpl>, user_id: LoginUserId) -> ApiResult<Response> {
    let user = state
        .user_application_service()
        .get(&user_id)
        .await?
        .ok_or(UserApplicationError::UserNotExist(user_id.to_string()))?;

    let result = UserResponse::from(user);

    Ok(Json(result).into_response())
}

#[tracing::instrument(skip(state), err)]
async fn get_favorites(state: State<AppStateImpl>, user_id: LoginUserId) -> ApiResult<Response> {
    let stock_id_list = state
        .favorite_service()
        .get_all(&user_id)
        .await?
        .iter()
        .map(|favo| favo.stock_id.to_string())
        .collect();

    let result = state
        .company_query_service()
        .find_list(stock_id_list)
        .await?;
    let result: Vec<FavoriteResponse> = result
        .into_iter()
        .map(|c| FavoriteResponse::from(c))
        .collect();

    Ok(Json(result).into_response())
}

#[tracing::instrument(skip(state), err)]
async fn insert_favorite(
    state: State<AppStateImpl>,
    user_id: LoginUserId,
    Path(stock_id): Path<String>,
) -> ApiResult<Response> {
    let favorite = FavoriteData::new(user_id.to_string(), stock_id);
    state.favorite_service().add(favorite).await?;

    Ok(Json(serde_json::json!({
        "message": "succeed in regist favorite"
    }))
    .into_response())
}

#[tracing::instrument(skip(state), err)]
async fn delete_favorite(
    state: State<AppStateImpl>,
    user_id: LoginUserId,
    Path(stock_id): Path<String>,
) -> ApiResult<Response> {
    let favorite = FavoriteData::new(user_id.to_string(), stock_id);
    state.favorite_service().remove(favorite).await?;

    Ok(Json(serde_json::json!({
        "message": "succeed in delete favorite"
    }))
    .into_response())
}

#[tracing::instrument(skip(state), err)]
async fn get_portfolio(state: State<AppStateImpl>, user_id: LoginUserId) -> ApiResult<Response> {
    let portfolio = state.portfolio_service().get_all(&user_id).await?;

    let result = portfolio
        .into_iter()
        .map(|p| PortfolioResponse::from(p))
        .collect();
    let result: Vec<PortfolioResponse> = result;

    Ok(Json(result).into_response())
}

#[tracing::instrument(skip(state), err)]
async fn insert_portfolio(
    state: State<AppStateImpl>,
    user_id: LoginUserId,
    Path(stock_id): Path<String>,
) -> ApiResult<Response> {
    let portfolio = PortfolioData::new(user_id.to_string(), stock_id);
    state.portfolio_service().add(portfolio).await?;

    Ok(Json(serde_json::json!({
        "message": "succeed in regist portfolio"
    }))
    .into_response())
}

#[tracing::instrument(skip(state), err)]
async fn update_portfolio(
    state: State<AppStateImpl>,
    user_id: LoginUserId,
    Path(stock_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResult<Response> {
    let purchase = if let Some(purchase) = params.get("purchase") {
        let Ok(purchase) = purchase.parse() else {return Err(PortfolioApplicationError::InvalidParameter { name: "purchase", value: purchase.clone() }.into()) };
        Some(purchase)
    } else {
        None
    };
    let stock_count = if let Some(stock_count) = params.get("stock_count") {
        let Ok(stock_count) = stock_count.parse() else {return Err(PortfolioApplicationError::InvalidParameter { name: "stock_count", value: stock_count.clone() }.into())};
        Some(stock_count)
    } else {
        None
    };
    let command = PortfolioUpdateCommand::new(user_id.to_string(), stock_id, purchase, stock_count);

    state.portfolio_service().update(command).await?;

    Ok(Json(serde_json::json!({
        "message": "succeed in update portfolio"
    }))
    .into_response())
}

#[tracing::instrument(skip(state), err)]
async fn delete_portfolio(
    state: State<AppStateImpl>,
    user_id: LoginUserId,
    Path(stock_id): Path<String>,
) -> ApiResult<Response> {
    state
        .portfolio_service()
        .remove(&user_id, &stock_id)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "succeed in update portfolio"
    }))
    .into_response())
}
