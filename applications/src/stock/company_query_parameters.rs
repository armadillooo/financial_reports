#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanyQueryParameters {
    pub name: Option<String>,
    pub stock_id: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}
