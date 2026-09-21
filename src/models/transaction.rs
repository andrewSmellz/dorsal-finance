use chrono::{DateTime,Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct Transaction{
    id: Uuid,
    account_id: Uuid,
    kind: TransactionKind,
    amount: Decimal,
    timestamp: DateTime<Utc>,


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

impl Transaction{
    pub fn new(account_id: Uuid, kind:TransactionKind, amount:Decimal) -> Self{
       Transaction{
        id: Uuid::new_v4(),
        account_id,
        kind,
        amount,
        timestamp: Utc::now(),
       } 
    }
}