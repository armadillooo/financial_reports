use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockQueryError {
    #[error("internal server error")]
    Disconnect,
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: String, value: String },
    #[error("start date exceeds end date")]
    InvalidRangeOfDate { start: NaiveDate, end: NaiveDate },
    #[error("stock data not found")]
    NotFound,
}

pub type StockQueryResult<T> = Result<T, StockQueryError>;
