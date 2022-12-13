use anyhow::anyhow;

use super::stock_data::StockData;
use super::stock_query_parameters::StockQueryParameters;
use super::stock_query_service::StockQueryService;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InmemoryStockQueryServiceImpl {
    pub stocks: Vec<StockData>,
}

impl InmemoryStockQueryServiceImpl {
    /// コンストラクタ
    pub fn new() -> Self {
        Self { stocks: Vec::new() }
    }
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
        let find_by_id = |s: &StockData| &s.stock_id == &param.stock_id;
        // 日付範囲指定(下限)
        let find_by_date_from = |s: &StockData| {
            if let Some(from) = &param.date_from {
                &s.date >= from
            } else {
                true
            }
        };
        // 日付範囲指定(上限)
        let find_by_date_to = |s: &StockData| {
            if let Some(to) = &param.date_to {
                &s.date <= to
            } else {
                true
            }
        };
        // ページ番号指定
        let page_index = if let Some(page) = param.page {
            page - 1
        } else {
            0
        };
        // ページサイズ指定
        let page_size = if let Some(size) = param.size {
            size as usize
        } else {
            self.stocks.len()
        };

        let iter = self
            .stocks
            .to_vec()
            .into_iter()
            .filter(find_by_id)
            .filter(find_by_date_from)
            .filter(find_by_date_to)
            .skip(page_index as usize * page_size)
            .take(page_size);

        Ok(iter.collect::<Vec<StockData>>())
    }
}

#[cfg(test)]
mod test {
    use crate::stock::{
        stock_data::StockData, stock_query_parameters::StockQueryParameters,
        stock_query_service::StockQueryService,
    };

    use super::InmemoryStockQueryServiceImpl;

    fn setup() -> InmemoryStockQueryServiceImpl {
        InmemoryStockQueryServiceImpl::new()
    }

    #[tokio::test]
    async fn find_by_id() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryParameters::new();
        let target_id = "2";
        param.stock_id = target_id.to_string();

        let mut stocks = Vec::new();
        for i in 0..3 {
            stocks.insert(i, StockData::new());
            stocks[i].stock_id = i.to_string();
        }
        service.stocks = stocks;

        assert!(service.find(param).await?.pop().unwrap().stock_id == target_id);

        Ok(())
    }
}
