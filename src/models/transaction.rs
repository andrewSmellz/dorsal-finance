use chrono::{DateTime,Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use super::Currency;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: TransactionKind,
    pub amount: Money,
    pub position: Option<Position>,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money {
    pub value: Decimal,
    pub currency: Currency,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position  {
    pub ticker: String,
    pub quantity: Decimal,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum TransactionKind{
    Deposit,
    Withdrawal,
    Income,
    Expense,

    Buy,
    Sell,
    Dividend,
    Interest,

    TransferOut,
    TransferIn,
}

pub enum CashKind{
    Deposit,
    Withdrawal,
    Income,
    Expense,
    Dividend,
    Interest,
}


impl CashKind{
    pub fn is_outflow(&self) -> bool{
        matches!(self, CashKind::Withdrawal | CashKind::Expense)
    }
}

impl From<CashKind> for TransactionKind {
    fn from(k: CashKind) -> Self {
        match k {
            CashKind::Deposit => TransactionKind::Deposit,
            CashKind::Withdrawal => TransactionKind::Withdrawal,
            CashKind::Income => TransactionKind::Income,
            CashKind::Expense => TransactionKind::Expense,
            CashKind::Dividend => TransactionKind::Dividend,
            CashKind::Interest => TransactionKind::Interest,
        }
    }
}


impl Money {
    pub fn new(value: Decimal, currency: Currency) -> Self {
        Money { value, currency }
    }
}


impl Transaction {
    pub fn new(account_id: Uuid, kind: TransactionKind, amount: Money) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            account_id,
            kind,
            amount,
            position: None,
            timestamp: Utc::now(),
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_position(mut self, ticker: impl Into<String>, quantity: Decimal) -> Self {
        self.position = Some(Position { ticker: ticker.into(), quantity });
        self
    }
}