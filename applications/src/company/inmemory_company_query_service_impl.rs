use std::ops::Deref;

use crate::company::{
    CompanyData, CompanyQueryCommand, CompanyQueryError, CompanyQueryResult, CompanyQueryService,
};
use domain::stock::StockId;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InmemoryCompanyQueryServiceImpl {
    pub companies: Vec<CompanyData>,
}

impl InmemoryCompanyQueryServiceImpl {
    /// コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl CompanyQueryService for InmemoryCompanyQueryServiceImpl {
    #[tracing::instrument(skip(self), err)]
    async fn find(&self, param: CompanyQueryCommand) -> CompanyQueryResult<Vec<CompanyData>> {
        // 企業名検索
        if let Some(name) = param.name {
            if let Some(company) = self.companies.iter().find(|c| c.name == name) {
                return Ok(vec![company.clone()]);
            } else {
                return Ok(vec![]);
            };
        }
        // ID検索
        if let Some(id) = param.stock_id {
            if let Some(company) = self.companies.iter().find(|c| c.stock_id.deref() == &id) {
                return Ok(vec![company.clone()]);
            } else {
                return Ok(vec![]);
            };
        }
        // セクター指定
        let find_by_sector = |c: &CompanyData| {
            if let Some(sector) = &param.sector {
                &c.sector == sector
            } else {
                true
            }
        };
        // 産業種別指定
        let find_by_industry = |c: &CompanyData| {
            if let Some(industry) = &param.industry {
                &c.industry == industry
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
            self.companies.len()
        };

        let iter = self
            .companies
            .to_vec()
            .into_iter()
            .filter(find_by_sector)
            .filter(find_by_industry)
            .skip(page_index as usize * page_size)
            .take(page_size);

        Ok(iter.collect::<Vec<CompanyData>>())
    }

    #[tracing::instrument(skip(self), err, ret)]
    async fn find_by_id(&self, stock_id: String) -> CompanyQueryResult<CompanyData> {
        let stock_id = StockId::new(stock_id);
        let result = self
            .companies
            .iter()
            .find(|c| c.stock_id == stock_id)
            .ok_or(CompanyQueryError::CompanyNotFound(stock_id.into()))?
            .clone();

        Ok(result)
    }

    #[tracing::instrument(skip(self), err)]
    async fn find_list(&self, stock_id_list: Vec<String>) -> CompanyQueryResult<Vec<CompanyData>> {
        let mut result = vec![];
        for id in stock_id_list {
            let found = self.find_by_id(id).await?;
            result.push(found);
        }

        Ok(result)
    }
}
