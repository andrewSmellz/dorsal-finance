use crate::models::Currency;
use rust_decimal::Decimal;
use thiserror::Error;
use std::sync::Arc;

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
    InsufficientShares {
        ticker: Arc<str>,
        requested: Decimal,
        held: Decimal,
    },

    #[error("amount must be greater than zero")]
    InvalidAmount,

    #[error("cannot transfer into the same account")]
    SameAccount,

    #[error("no usable price for {0}")]
    NoPrice(String),

    #[error("currency mismatch: expected {expected}, found {found}")]
    CurrencyMismatch { expected: Currency, found: Currency },

    #[error("unknown currency code {code} for {ticker}")]
    UnknownCurrency { code: String, ticker: String },

    #[error("invalid transaction kind in database: {0}")]
    InvalidKind(String),

    #[error("account {0} not found")]
    AccountNotFound(uuid::Uuid),

    #[error("invalid category name: {0:?}")]
    InvalidCategoryName(String),

    #[error("category {0:?} already exists")]
    DuplicateCategory(String),

    #[error("expenses and refunds need a category")]
    MissingCategory,

    #[error("only expenses and refunds can have a category")]
    UnexpectedCategory,

    #[error("category {0} not found")]
    CategoryNotFound(uuid::Uuid),

    #[error("category {0} is archived")]
    CategoryArchived(uuid::Uuid),

    #[error("category {0} is used by transactions; archive it instead")]
    CategoryInUse(uuid::Uuid),

    #[error("category {0} has subcategories; delete or move them first")]
    CategoryHasChildren(uuid::Uuid),

    #[error("a category can't be moved under itself or one of its own subcategories")]
    CategoryCycle,
}
