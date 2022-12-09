use anyhow::anyhow;

use super::stock_data::StockData;
use super::stock_query_parameters::StockQueryParameters;
use super::stock_query_service::StockQueryService;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InmemoryStockQueryServiceImpl {
    pub stocks: Vec<StockData>,
}

#[async_trait::async_trait]
impl StockQueryService for InmemoryStockQueryServiceImpl {
    async fn find(&self, param: StockQueryParameters) -> anyhow::Result<Vec<StockData>> {
        // パラメータチェック
        if let (Some(from), Some(to)) = (param.date_from, param.date_to) {
            if from > to {
                return Err(anyhow!("parameter error!"));
            }
        };
        // ID検索
        let filter_by_id = |s: &StockData| &s.stock_id == &param.stock_id;
        // 日付範囲(下限)
        let filter_by_date_from = |s: &StockData| {
            if let Some(from) = &param.date_from {
                &s.date >= from
            } else {
                true
            }
        };
        // 日付範囲(上限)
        let filter_by_date_to = |s: &StockData| {
            if let Some(to) = &param.date_to {
                &s.date <= to
            } else {
                true
            }
        };
        // ページ番号
        let page_index = if let Some(page) = param.page {
            page - 1
        } else {
            0
        };
        // ページサイズ
        let page_size = if let Some(size) = param.size {
            size as usize
        } else {
            self.stocks.len()
        };

        let iter = self
            .stocks
            .to_vec()
            .into_iter()
            .filter(filter_by_id)
            .filter(filter_by_date_from)
            .filter(filter_by_date_to)
            .skip(page_index as usize * page_size)
            .take(page_size);

        Ok(iter.collect::<Vec<StockData>>())
    }
}
