use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};

use crate::common::{Utility, UtilityImpl};
use applications::{
    favorite::{FavoriteData, FavoriteService},
    portfolio::{PortfolioData, PortfolioService, PortfolioUpdateCommand},
    users::UserService,
};

pub fn user_controller() -> Router {
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
        );

    Router::new().nest("/:user_id", user_route)
}

async fn get_user(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let user = utility
        .user_application_service()
        .get(&user_id)
        .await
        .unwrap();

    "Ok"
}

async fn get_favorites(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let favorites = utility.favorite_service().get_all(&user_id).await.unwrap();

    "Ok"
}

async fn insert_favorite(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let favorite = FavoriteData::new(user_id, stock_id);
    utility.favorite_service().add(favorite).await.unwrap();

    "Ok"
}

async fn delete_favorite(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let favorite = FavoriteData::new(user_id, stock_id);
    utility.favorite_service().remove(favorite).await.unwrap();

    "Ok"
}

async fn get_portfolio(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let portfolio = utility.portfolio_service().get_all(&user_id).await.unwrap();

    "Ok"
}

async fn insert_portfolio(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    let portfolio = PortfolioData::new(user_id, stock_id);
    utility.portfolio_service().add(portfolio).await.unwrap();

    "Ok"
}

async fn update_portfolio(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
    Path(stock_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let purchase = if let Some(purchase) = params.get("purchase") {
        let Ok(purchase) = purchase.parse() else {return "Err"};
        Some(purchase)
    } else {
        None
    };
    let stock_count = if let Some(stock_count) = params.get("purchase") {
        let Ok(stock_count) = stock_count.parse() else {return "Err"};
        Some(stock_count)
    } else {
        None
    };
    let command = PortfolioUpdateCommand::new(user_id, stock_id, purchase, stock_count);

    utility.portfolio_service().update(command).await.unwrap();

    "Ok"
}

async fn delete_portfolio(
    Extension(utility): Extension<UtilityImpl>,
    Path(user_id): Path<String>,
    Path(stock_id): Path<String>,
) -> impl IntoResponse {
    utility
        .portfolio_service()
        .remove(&user_id, &stock_id)
        .await
        .unwrap();

    "Ok"
}
