use sqlx::postgres::PgPool;

use domain::user::{User, UserDomainResult, UserEmail, UserId, UserName, UserRepository};

#[derive(Clone, Debug)]
pub struct PostgresUserRepositoryImpl {
    connection: PgPool,
}

impl PostgresUserRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepositoryImpl {
    async fn find(&self, id: &UserId) -> UserDomainResult<Option<User>> {
        let result = sqlx::query_as!(UserModel, r#"select * from users where id=$1"#, id.as_str())
            .fetch_optional(&self.connection)
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .map(|user| user.into());

        Ok(result)
    }

    async fn find_by_name(&self, name: &UserName) -> UserDomainResult<Option<User>> {
        let result = sqlx::query_as!(
            UserModel,
            r#"select * from users where name=$1"#,
            name.as_str()
        )
        .fetch_optional(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .map(|user| user.into());

        Ok(result)
    }

    async fn save(&self, user: User) -> UserDomainResult<()> {
        sqlx::query!(
            r#"insert into users values ($1, $2, $3)"#,
            user.id().as_str(),
            user.name().as_str(),
            user.email().as_str(),
        )
        .execute(&self.connection)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }

    async fn delete(&self, user: User) -> UserDomainResult<()> {
        sqlx::query!(r#"delete from users where id=$1"#, user.id().as_str(),)
            .execute(&self.connection)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct UserModel {
    id: String,
    name: String,
    email: String,
}

impl Into<User> for UserModel {
    fn into(self) -> User {
        let id = UserId::new(self.id);
        let name = UserName::new(self.name);
        let email = UserEmail::new(self.email);

        User::new(id, name, email)
    }
}
