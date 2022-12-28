use std::ops::Deref;

use crate::company::{
    CompanyData, CompanyQueryCommand, CompanyQueryError, CompanyQueryResult, CompanyQueryService,
};

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

        let result: Vec<CompanyData> = iter.collect();
        tracing::info!("company data count = {}", result.len());
        Ok(result)
    }

    #[tracing::instrument(skip(self), err, ret)]
    async fn find_by_id(&self, stock_id: &str) -> CompanyQueryResult<CompanyData> {
        let result = self
            .companies
            .iter()
            .find(|c| c.stock_id == stock_id)
            .ok_or(CompanyQueryError::CompanyNotFound(stock_id.to_string()))?
            .clone();

        Ok(result)
    }

    #[tracing::instrument(skip(self), err, ret)]
    async fn find_list(&self, stock_id_list: Vec<String>) -> CompanyQueryResult<Vec<CompanyData>> {
        let mut result = vec![];
        for id in stock_id_list {
            let found = self.find_by_id(&id).await?;
            result.push(found);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use crate::company::{
        CompanyData, CompanyQueryCommand, CompanyQueryError, CompanyQueryService,
        InmemoryCompanyQueryServiceImpl,
    };

    fn setup() -> impl CompanyQueryService {
        let mut service = InmemoryCompanyQueryServiceImpl::new();
        let sample_data = vec![
            CompanyData {
                name: "AGC".to_string(),
                stock_id: "1111".to_string(),
                sector: "旅館".to_string(),
                industry: "化学工業".to_string(),
            },
            CompanyData {
                name: "KDDI".to_string(),
                stock_id: "2222".to_string(),
                sector: "化学繊維製造業".to_string(),
                industry: "化学工業".to_string(),
            },
            CompanyData {
                name: "T&Dホールディングス".to_string(),
                stock_id: "3333".to_string(),
                sector: "人材派遣業".to_string(),
                industry: "鉱業".to_string(),
            },
            CompanyData {
                name: "いすゞ自動車".to_string(),
                stock_id: "4444".to_string(),
                sector: "製氷業".to_string(),
                industry: "建設業".to_string(),
            },
            CompanyData {
                name: "りそなホールディングス".to_string(),
                stock_id: "5555".to_string(),
                sector: "化学繊維製造業".to_string(),
                industry: "製造業".to_string(),
            },
        ];

        service.companies = sample_data;

        service
    }

    #[tokio::test]
    async fn find_by_name() -> anyhow::Result<()> {
        let service = setup();
        let mut command = CompanyQueryCommand::new();
        command.name = Some("いすゞ自動車".to_string());

        assert!(service.find(command).await?.pop().unwrap().name == "いすゞ自動車");

        Ok(())
    }

    #[tokio::test]
    async fn find_by_id_command() -> anyhow::Result<()> {
        let service = setup();
        let mut command = CompanyQueryCommand::new();
        command.stock_id = Some("5555".to_string());

        assert!(service.find(command).await?.pop().unwrap().stock_id == "5555");

        Ok(())
    }

    #[tokio::test]
    async fn find_by_id() -> anyhow::Result<()> {
        let service = setup();
        assert!(service.find_by_id("4444").await?.stock_id == "4444");

        Ok(())
    }

    #[tokio::test]
    async fn find_notexist_id_return_err() -> anyhow::Result<()> {
        let service = setup();
        let Err(CompanyQueryError::CompanyNotFound(_)) = service.find_by_id("not exits id").await else {
            return Err(anyhow!("unexpected stock query result"));
        };

        Ok(())
    }

    #[tokio::test]
    async fn find_by_sector() -> anyhow::Result<()> {
        let service = setup();
        let mut command = CompanyQueryCommand::new();
        command.sector = Some("化学繊維製造業".to_string());

        assert!(service.find(command.clone()).await?.pop().unwrap().sector == "化学繊維製造業");
        assert!(service.find(command).await?.pop().unwrap().sector == "化学繊維製造業");

        Ok(())
    }

    #[tokio::test]
    async fn find_by_industry() -> anyhow::Result<()> {
        let service = setup();
        let mut command = CompanyQueryCommand::new();
        command.industry = Some("化学工業".to_string());

        assert!(service.find(command.clone()).await?.pop().unwrap().industry == "化学工業");
        assert!(service.find(command).await?.pop().unwrap().industry == "化学工業");

        Ok(())
    }

    #[tokio::test]
    async fn pagenation() -> anyhow::Result<()> {
        let service = setup();
        let mut param = CompanyQueryCommand::new();
        let index = Some(2);
        let page_size = Some(1);
        param.page = index;
        param.size = page_size;

        let found = service.find(param).await?;
        assert!(found.len() as i32 == page_size.unwrap());
        assert!(&found[0].stock_id == "2222");

        Ok(())
    }

    #[tokio::test]
    async fn find_list() -> anyhow::Result<()> {
        let service = setup();
        let id_list = vec!["1111".to_string(), "2222".to_string(), "3333".to_string()];
        let result = service.find_list(id_list.clone()).await?;
        let result_id_list: Vec<String> = result.into_iter().map(|c| c.stock_id).collect();

        assert!(result_id_list == id_list);

        Ok(())
    }
}
