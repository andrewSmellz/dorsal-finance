use chrono::{DateTime, Local, NaiveDate, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use super::Currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: Uuid,
    pub transfer_id: Option<Uuid>,
    pub account_id: Uuid,
    pub kind: TransactionKind,
    pub amount: Money,
    pub position: Option<Position>,
    pub occurred_on: NaiveDate,
    pub created_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionKind{
    Deposit,
    Withdrawal,
    Income,
    Expense(ExpenseCategory),
    Refund(ExpenseCategory),
    Buy,
    Sell,
    Dividend,
    Interest,

    TransferOut,
    TransferIn,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashKind{
    Deposit,
    Withdrawal,
    Income,
    Expense(ExpenseCategory),
    Refund(ExpenseCategory),
    Dividend,
    Interest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpenseCategory{
    Housing,
    Utilities,
    Phone,
    Insurance,
    HealthCare,
    Grocery(GroceryKind),
    Shopping(ShoppingKind),
    Transportation,
    EatingOut,
    Entertainment,
    Hobbies,
    Gifts,
    Misc
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroceryKind {
    Meat,
    Produce,
    Dairy,
    Carb,
    JunkFood,
    Alcohol,
    Misc
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShoppingKind{
    Clothes,
    Electronics,
    Misc,
}

impl CashKind{
    pub fn is_outflow(&self) -> bool{
        matches!(self, CashKind::Withdrawal | CashKind::Expense(_))
    }
}

impl From<CashKind> for TransactionKind {
    fn from(k: CashKind) -> Self {
        match k {
            CashKind::Deposit => TransactionKind::Deposit,
            CashKind::Withdrawal => TransactionKind::Withdrawal,
            CashKind::Income => TransactionKind::Income,
            CashKind::Expense(category) => TransactionKind::Expense(category),
            CashKind::Refund(category) => TransactionKind::Refund(category),
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

impl Position{
    pub fn new( ticker: String,quantity: Decimal,) -> Self{
        Position { ticker, quantity }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}

impl Transaction {
    pub fn new(account_id: Uuid, kind: TransactionKind, amount: Money) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            transfer_id: None,
            account_id,
            kind,
            amount,
            position: None,
            occurred_on: Local::now().date_naive(),
            created_at: Utc::now(),
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_position(mut self, ticker: impl Into<String>, quantity: Decimal) -> Self {
        self.position = Some(Position::new(ticker.into(), quantity));
        self
    }

    pub fn with_occurred_on(mut self, date: NaiveDate) -> Self {
        self.occurred_on = date;
        self
    }
}