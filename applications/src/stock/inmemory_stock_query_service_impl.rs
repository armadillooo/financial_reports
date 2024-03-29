use std::ops::Deref;

use crate::stock::{
    StockData, StockQueryCommand, StockQueryError, StockQueryResult, StockQueryService,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
    #[tracing::instrument(skip(self), err)]
    async fn find(&self, param: StockQueryCommand) -> StockQueryResult<Vec<StockData>> {
        // パラメータチェック
        if let (Some(start), Some(end)) = (param.start, param.end) {
            if start > end {
                return Err(StockQueryError::InvalidRangeOfDate {
                    name: "end",
                    value: end,
                });
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
            if let Some(from) = &param.start {
                &s.date >= from
            } else {
                true
            }
        };
        // 日付範囲指定(上限)
        let find_by_date_to = |s: &StockData| {
            if let Some(to) = &param.end {
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

    #[tracing::instrument(skip(self), err, ret)]
    async fn find_latest(&self, stock_id: &str) -> StockQueryResult<StockData> {
        let mut command = StockQueryCommand::new();
        command.stock_id = Some(stock_id.to_string());

        let latest = self
            .find(command)
            .await?
            .into_iter()
            .max_by(|s1, s2| s1.date.cmp(&s2.date))
            .ok_or(StockQueryError::StockDataNotFound(stock_id.into()))?;

        Ok(latest)
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use chrono::NaiveDate;

    use crate::stock::{
        inmemory_stock_query_service_impl::InmemoryStockQueryServiceImpl, stock_data::StockData,
        stock_query_command::StockQueryCommand, stock_query_service::StockQueryService,
    };

    fn setup() -> InmemoryStockQueryServiceImpl {
        InmemoryStockQueryServiceImpl::new()
    }

    #[tokio::test]
    async fn find_by_id() -> anyhow::Result<()> {
        let mut service = setup();
        let mut param = StockQueryCommand::new();
        let target_id = "2";
        param.stock_id = Some(target_id.to_string());

        let mut stocks = Vec::new();
        for i in 0..3 {
            stocks.insert(i, StockData::new());
            stocks[i].stock_id = i.to_string();
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
        let mut param = StockQueryCommand::new();
        let target_date = NaiveDate::from_ymd_opt(2022, 7, 12);
        param.start = target_date;

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
        let mut param = StockQueryCommand::new();
        let target_date = NaiveDate::from_ymd_opt(2022, 7, 12);
        param.end = target_date;

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
        let mut param = StockQueryCommand::new();
        let index = Some(2);
        let page_size = Some(1);
        param.page = index;
        param.size = page_size;

        let mut stocks = Vec::new();
        for i in 0..3 {
            stocks.insert(i, StockData::new());
            stocks[i].stock_id = i.to_string();
        }
        service.stocks = stocks;

        let found = service.find(param).await?;
        assert!(found.len() as i32 == page_size.unwrap());
        assert!(found[0].stock_id == (index.unwrap() - 1).to_string());

        Ok(())
    }

    #[tokio::test]
    async fn find_latest_stock_data() -> anyhow::Result<()> {
        let mut service = setup();
        let target_date = NaiveDate::from_ymd_opt(2099, 7, 12).unwrap();
        let stock_id = "1234";

        let mut stocks = Vec::new();
        for _ in 0..4 {
            stocks.push(StockData::new());
        }
        stocks[0].date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        stocks[0].stock_id = stock_id.to_string();
        stocks[1].date = target_date.clone();
        stocks[1].stock_id = stock_id.to_string();
        stocks[2].date = NaiveDate::from_ymd_opt(2022, 8, 30).unwrap();
        stocks[2].stock_id = stock_id.to_string();
        stocks[3].date = NaiveDate::from_ymd_opt(2025, 9, 15).unwrap();
        stocks[3].stock_id = stock_id.to_string();
        service.stocks = stocks;

        let found = service.find_latest(stock_id).await?;

        assert!(found.date == target_date);

        Ok(())
    }
}
