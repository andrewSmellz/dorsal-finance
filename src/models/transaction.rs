use chrono::{DateTime,Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: TransactionKind,
    pub amount: Decimal,
    pub ticker: Option<String>,
    pub quantity: Option<Decimal>,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
}

pub enum TransactionKind{
    Deposit,
    Withdrawal,
    Income,
    Expense,

    Buy,
    Sell,
    Dividend,
    Interest,
}

impl Transaction {
    pub fn new(account_id: Uuid, kind: TransactionKind, amount: Decimal) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            account_id,
            kind,
            amount,
            ticker: None,
            quantity: None,
            timestamp: Utc::now(),
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn with_quantity(mut self, quantity: Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }
}