use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockQueryError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: String, value: String },
    #[error("start date exceeds end date")]
    InvalidRangeOfDate { start: NaiveDate, end: NaiveDate },
    #[error("stock data not found")]
    StockDataNotFound,
}

pub type StockQueryResult<T> = Result<T, StockQueryError>;
