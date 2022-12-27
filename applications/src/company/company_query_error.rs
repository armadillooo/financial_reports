use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompanyQueryError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: &'static str, value: String },
    #[error("company data not found: id={0}")]
    CompanyNotFound(String),
}

pub type CompanyQueryResult<T> = Result<T, CompanyQueryError>;
