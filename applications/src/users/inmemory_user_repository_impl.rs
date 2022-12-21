use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use domain::users::{User, UserDomainError, UserDomainResult, UserId, UserName, UserRepository};

/// テスト用Userレポジトリ
#[derive(Debug, Clone, Default)]
pub struct InMemoryUserRepositoryImpl {
    store: Arc<Mutex<HashMap<String, User>>>,
}

impl InMemoryUserRepositoryImpl {
    /// コンストラクタ
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::<String, User>::new())),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepositoryImpl {
    /// ユーザー削除
    async fn delete(&self, user: User) -> UserDomainResult<()> {
        self.store.lock().unwrap().remove(&user.id().to_string());

        Ok(())
    }

    async fn find(&self, id: &UserId) -> UserDomainResult<Option<User>> {
        if let Some(user) = self.store.lock().unwrap().get(&id.to_string()) {
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }

    async fn find_by_name(&self, name: &UserName) -> UserDomainResult<Option<User>> {
        if let Some(user) = self
            .store
            .lock()
            .unwrap()
            .values()
            .find(|val| val.name() == name)
        {
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, user: User) -> UserDomainResult<()> {
        let key = user.id().to_string();
        if self.store.lock().unwrap().contains_key(&key) {
            return Err(UserDomainError::UserAlreadyExist);
        };

        self.store.lock().unwrap().insert(key, user);

        Ok(())
    }
}
