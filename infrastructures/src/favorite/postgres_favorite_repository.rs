use sqlx::postgres::PgPool;

use domain::{
    favorite::{Favorite, FavoriteDomainResult, FavoriteRepository},
    stock::StockId,
    user::UserId,
};

pub struct PosgtresFavoriteRepository {
    connection: PgPool,
}

#[async_trait::async_trait]
impl FavoriteRepository for PosgtresFavoriteRepository {
    async fn save(&self, favorite: Favorite) -> FavoriteDomainResult<()> {
        sqlx::query!(
            r#"
            insert into favorites values ($1, $2)
            "#,
            favorite.user_id.as_str(),
            favorite.stock_id.as_str(),
        )
        .execute(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    async fn delete(&self, favorite: Favorite) -> FavoriteDomainResult<()> {
        sqlx::query!(
            r#"delete from favorites where user_id=$1"#,
            favorite.user_id.as_str(),
        )
        .execute(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    async fn find_all(&self, user_id: &UserId) -> FavoriteDomainResult<Vec<Favorite>> {
        let result = sqlx::query_as!(
            FavoriteModel,
            r#"select * from favorites where user_id=$1"#,
            user_id.as_str()
        )
        .fetch_all(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        let result = result.into_iter().map(|favo| favo.into()).collect();
        Ok(result)
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct FavoriteModel {
    user_id: String,
    stock_id: String,
}

impl Into<Favorite> for FavoriteModel {
    fn into(self) -> Favorite {
        Favorite {
            user_id: UserId::new(self.user_id),
            stock_id: StockId::new(self.stock_id),
        }
    }
}
