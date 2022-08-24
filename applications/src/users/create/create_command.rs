pub use crate::users::UserData;

pub struct CreateCommand {
    pub user: UserData,
}

impl CreateCommand {
    /// コンストラクタ
    pub fn new(user: UserData) -> Self {
        Self { user }
    }
}
