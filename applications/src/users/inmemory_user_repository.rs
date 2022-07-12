use std::collections::HashMap;
use std::sync::Mutex;

use domain::users::{User, UserId, UserName, UserRepository};

/// テスト用Userレポジトリ
#[derive(Debug)]
pub struct InMemoryUserRepository {
    pub store: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::<String, User>::new()),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    /// ユーザー削除
    fn delete(&self, user: User) -> anyhow::Result<()> {
        if let Some(_) = self.store.lock().unwrap().remove(user.id().value()) {
            Ok(())
        } else {
            Err(anyhow::format_err!("User not exists"))
        }
    }

    fn find(&self, id: &UserId) -> anyhow::Result<Option<User>> {
        if let Some(user) = self.store.lock().unwrap().get(id.value()) {
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }

    fn find_by_name(&self, name: &UserName) -> anyhow::Result<Option<User>> {
        if let Some(user) = self.store.lock().unwrap().values().find(|val| val.name() == name) {
            Ok(Some(user.clone()))
        } else {
            Ok(None)
        }
    }

    fn save(&self, user: User) -> anyhow::Result<()> {
        let key = user.id().value();
        if self.store.lock().unwrap().contains_key(key) {
            return Err(anyhow::format_err!("User already exists"));
        };

        self.store.lock().unwrap().insert(key.to_string(), user);

        Ok(())
    }
}