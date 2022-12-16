use domain::stock::StockId;

/// 企業情報
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Companies {
    /// 企業名
    pub name: String,
    /// ID
    pub stock_id: StockId,
    /// 分類
    pub sector: String,
    // 産業
    pub industry: String,
}
