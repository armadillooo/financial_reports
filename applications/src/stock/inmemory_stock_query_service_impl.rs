use std::ops::Deref;

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
        let find_by_id = |s: &StockData| {
            if let Some(id) = &param.stock_id {
                &s.stock_id.deref() == &id
            } else {
                true
            }
        };
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
        let skip_count = page_index as usize * page_size;

        let iter = self
            .stocks
            .to_vec()
            .into_iter()
            .filter(find_by_id)
            .filter(find_by_date_from)
            .filter(find_by_date_to)
            .skip(skip_count)
            .take(page_size);

        let result = iter.collect::<Vec<StockData>>();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use chrono::NaiveDate;
    use domain::stock::StockId;

    use crate::stock::{
        inmemory_stock_query_service_impl::InmemoryStockQueryServiceImpl, stock_data::StockData,
        stock_query_parameters::StockQueryParameters, stock_query_service::StockQueryService,
    };

    fn setup() -> InmemoryStockQueryServiceImpl {
        InmemoryStockQueryServiceImpl::new()
    }

    #[tokio::test]
    async fn find_by_id() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryParameters::new();
        let target_id = "2";
        param.stock_id = Some(target_id.to_string());

        let mut stocks = Vec::new();
        for i in 0..3 {
            stocks.insert(i, StockData::new());
            stocks[i].stock_id = StockId::new(i.to_string());
        }
        service.stocks = stocks;

        let found = service.find(param).await?;

        assert!(found.len() == 1);
        assert!(found[0].stock_id.deref() == target_id);

        Ok(())
    }

    #[tokio::test]
    async fn find_by_date_from() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryParameters::new();
        let target_date = NaiveDate::from_ymd_opt(2022, 7, 12);
        param.date_from = target_date;

        let mut stocks = Vec::new();
        for i in 0..4 {
            stocks.insert(i, StockData::new());
        }
        stocks[0].date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        stocks[1].date = target_date.unwrap();
        stocks[2].date = NaiveDate::from_ymd_opt(2022, 8, 30).unwrap();
        stocks[3].date = NaiveDate::from_ymd_opt(2025, 9, 15).unwrap();
        service.stocks = stocks.to_vec();

        let found = service.find(param).await?;

        assert!(found == stocks[1..]);

        Ok(())
    }

    #[tokio::test]
    async fn find_by_date_to() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryParameters::new();
        let target_date = NaiveDate::from_ymd_opt(2022, 7, 12);
        param.date_to = target_date;

        let mut stocks = Vec::new();
        for i in 0..4 {
            stocks.insert(i, StockData::new());
        }
        stocks[0].date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        stocks[1].date = target_date.unwrap();
        stocks[2].date = NaiveDate::from_ymd_opt(2022, 8, 30).unwrap();
        stocks[3].date = NaiveDate::from_ymd_opt(2025, 9, 15).unwrap();
        service.stocks = stocks.to_vec();

        let found = service.find(param).await?;

        assert!(found == stocks[..2]);

        Ok(())
    }

    #[tokio::test]
    async fn pagenation() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryParameters::new();
        let index = Some(2);
        let page_size = Some(1);
        param.page = index;
        param.size = page_size;

        let mut stocks = Vec::new();
        for i in 0..3 {
            stocks.insert(i, StockData::new());
            stocks[i].stock_id = StockId::new(i.to_string());
        }
        service.stocks = stocks;

        let found = service.find(param).await?;
        assert!(found.len() as i32 == page_size.unwrap());
        assert!(found[0].stock_id == StockId::new((index.unwrap() - 1).to_string()));

        Ok(())
    }
}
