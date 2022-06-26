use std::{cell::RefCell, collections::HashMap, rc::Rc};

use domain::users::{User, UserId, UserName, UserRepository};

/// テスト用Userレポジトリ
pub struct InMemoryUserRepository {
    store: Rc<RefCell<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new(store: HashMap<String, User>) -> Self {
        Self {
            store: Rc::new(RefCell::new(store)),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    /// ユーザー削除
    fn delete(&self, user: User) -> anyhow::Result<()> {
        if let Some(_) = self.store.borrow_mut().remove(user.id().value()) {
            Ok(())
        } else {
            Err(anyhow::format_err!("User not exists"))
        }
    }

    fn find(&self, id: &UserId) -> anyhow::Result<User> {
        if let Some(user) = self.store.borrow().get(id.value()) {
            Ok(user.clone())
        } else {
            Err(anyhow::format_err!("User not found"))
        }
    }

    fn find_by_name(&self, name: &UserName) -> anyhow::Result<User> {
        if let Some(user) = self.store.borrow().values().find(|val| val.name() == name) {
            Ok(user.clone())
        } else {
            Err(anyhow::format_err!("User not found"))
        }
    }

    fn save(&self, user: &User) -> anyhow::Result<()> {
        if let Some(_name) = self
            .store
            .borrow_mut()
            .insert(user.id().value().to_string(), user.clone())
        {
            Ok(())
        } else {
            Err(anyhow::format_err!("User already exists"))
        }
    }
}
