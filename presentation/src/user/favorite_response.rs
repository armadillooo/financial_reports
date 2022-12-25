use applications::company::CompanyData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FavoriteResponse {
    pub stock_id: String,
    pub name: String,
    pub sector: String,
    pub industry: String,
}

impl From<CompanyData> for FavoriteResponse {
    fn from(value: CompanyData) -> Self {
        Self {
            name: value.name,
            stock_id: value.stock_id.to_string(),
            sector: value.sector,
            industry: value.industry,
        }
    }
}
