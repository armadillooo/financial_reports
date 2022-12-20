use domain::{favorite::Favorite, stock::StockId, users::UserId};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FavoriteData {
    pub user_id: String,
    pub stock_id: String,
}

impl FavoriteData {
    /// コンストラクタ
    pub fn new(user_id: String, stock_id: String) -> Self {
        Self { user_id, stock_id }
    }
}

impl From<Favorite> for FavoriteData {
    fn from(favorite: Favorite) -> Self {
        Self {
            user_id: favorite.user_id.to_string(),
            stock_id: favorite.stock_id.to_string(),
        }
    }
}

impl Into<Favorite> for FavoriteData {
    fn into(self) -> Favorite {
        Favorite {
            user_id: UserId::new(self.user_id),
            stock_id: StockId::new(self.stock_id),
        }
    }
}
