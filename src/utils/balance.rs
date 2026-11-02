use uuid::Uuid;
use rust_decimal::Decimal;

use crate::models::TransactionKind::*;
use crate::queries::transaction::list_transactions;

pub async fn account_balance<'e, E>(exec: E, account_id: Uuid) -> Decimal
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
    let txs = list_transactions(exec, account_id).await.unwrap();
    txs.iter().fold(Decimal::ZERO, |bal,tx| 
    match tx.kind{
        Deposit | Income | Sell | Dividend | Interest | TransferIn=> bal + tx.amount,
        Withdrawal | Expense | Buy | TransferOut=> bal - tx.amount,
    }
    )
}