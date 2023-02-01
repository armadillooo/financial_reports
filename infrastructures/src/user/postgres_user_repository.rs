use domain::user::{User, UserDomainResult, UserId, UserName, UserRepository};

pub struct PostgresUserRepository {}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find(&self, id: &UserId) -> UserDomainResult<Option<User>> {
        unimplemented!()
    }

    async fn find_by_name(&self, name: &UserName) -> UserDomainResult<Option<User>> {
        unimplemented!()
    }

    async fn save(&self, user: User) -> UserDomainResult<()> {
        unimplemented!()
    }

    async fn delete(&self, user: User) -> UserDomainResult<()> {
        unimplemented!()
    }
}
