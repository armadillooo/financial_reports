use crate::{stock::StockId, users::UserId};

/// お気に入り登録情報
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Favorite {
    pub user_id: UserId,
    pub companies: Vec<StockId>,
}
