use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompanyQueryError {
    #[error("internal server error")]
    Disconnect,
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: &'static str, value: String },
    #[error("company data not found")]
    CompanyNotFound,
}

pub type CompanyQueryResult<T> = Result<T, CompanyQueryError>;
