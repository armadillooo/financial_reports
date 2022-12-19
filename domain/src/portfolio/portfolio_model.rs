use crate::{stock::StockId, users::UserId};

// ポートフォリオ情報
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Portfolio {
    /// ユーザーID
    pub user_id: UserId,
    /// 株価ID
    pub stock_id: StockId,
    /// 保有株数
    pub stock_count: i32,
    /// 1株当たり購入価格(円)
    pub purchase: i32,
}

impl Portfolio {
    pub fn update_stock_count(&mut self, stock_count: i32) {
        self.stock_count = stock_count;
    }

    pub fn update_purchase(&mut self, purchase: i32) {
        self.purchase = purchase;
    }
}
