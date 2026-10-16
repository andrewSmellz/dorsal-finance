use sqlx::SqlitePool;
use uuid::Uuid;
use rust_decimal::{Decimal};

use crate::models::TransactionKind::*;
use crate::queries::transaction::list_transactions;

pub async fn account_balance(pool: &SqlitePool, account_id: Uuid) -> Decimal{
    let txs = list_transactions(pool, account_id).await.unwrap();
    txs.iter().fold(Decimal::ZERO, |bal,tx| 
    match tx.kind{
        Deposit | Income | Sell | Dividend | Interest | TransferIn=> bal + tx.amount,
        Withdrawal | Expense | Buy | TransferOut=> bal - tx.amount,
    }
    )
}