use rust_decimal::prelude::Zero;
use uuid::Uuid;
use rust_decimal::Decimal;


use crate::models::TransactionKind::*;
use crate::queries::transaction::{list_transactions,list_all_transactions};

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

pub async fn net_worth<'e, E>(exec: E, account_id: Uuid) -> Decimal
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let txs = list_all_transactions(exec).await.unwrap();
    //TODO finish off networth, need a way to calculate stock value for this
    //afterwards call account balance and this new stock balance? function
    //then just sum up everything

    Decimal::zero()
}