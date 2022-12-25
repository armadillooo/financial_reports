use applications::company::CompanyData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompanyResponse {
    name: String,
    stock_id: String,
    sector: String,
    industry: String,
}

impl From<CompanyData> for CompanyResponse {
    fn from(value: CompanyData) -> Self {
        Self {
            name: value.name,
            stock_id: value.stock_id.to_string(),
            sector: value.sector,
            industry: value.industry,
        }
    }
}
