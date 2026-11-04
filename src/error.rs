use rust_decimal::Decimal;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // wrapped errors from other crates; #[from] generates From<...> so `?` converts
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("price lookup failed: {0}")]
    Price(#[from] yahoo_finance_api::YahooError),

    #[error("invalid decimal in database: {0}")]
    InvalidDecimal(#[from] rust_decimal::Error),

    #[error("insufficient account balance")]
    InsufficientFunds,

    #[error("cannot sell {requested} of {ticker}, only {held} held")]
    InsufficientShares { ticker: String, requested: Decimal, held: Decimal },

    #[error("amount must be greater than zero")]
    InvalidAmount,

    #[error("cannot transfer into the same account")]
    SameAccount,
}