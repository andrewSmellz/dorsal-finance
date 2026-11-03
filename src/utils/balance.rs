use rust_decimal::prelude::Zero;
use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::SqlitePool;


use crate::models::TransactionKind::*;
use crate::queries::account::get_all_accounts;
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

pub async fn net_worth(pool: &SqlitePool) -> Decimal {
    let accounts = get_all_accounts(pool).await.unwrap();

    let mut networth = Decimal::ZERO;
    for account in &accounts {
        println!("account name:{}", account.name);
        let balance = account_balance(pool, account.id).await;
        networth += balance;
        println!("account balance:{}", balance);
    }
    networth
}