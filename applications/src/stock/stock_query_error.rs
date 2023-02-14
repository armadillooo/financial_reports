use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockQueryError {
    #[error(transparent)]
    Disconnect(#[from] anyhow::Error),
    #[error("invalid parameter: {name}={value}")]
    InvalidParameter { name: &'static str, value: String },
    #[error("invalid date parameter: {name}={value}")]
    InvalidRangeOfDate {
        name: &'static str,
        value: NaiveDate,
    },
    #[error("stock data not found: id={0}")]
    StockDataNotFound(String),
}

pub type StockQueryResult<T> = Result<T, StockQueryError>;
