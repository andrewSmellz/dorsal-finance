use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use yahoo_finance_api as yahoo;
use crate::error::{Error, Result};

use crate::models::TransactionKind::*;
use crate::queries::account::get_all_accounts;
use crate::queries::transaction::list_transactions;
use crate::utils::holding::{account_holdings, get_price_by_ticker};

pub async fn account_balance<'e, E>(exec: E, account_id: Uuid) -> Result<Decimal>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
    let txs = list_transactions(exec, account_id).await?;
   
    Ok(txs.iter().fold(Decimal::ZERO, |bal,tx| 
    match tx.kind{
        Deposit | Income | Sell | Dividend | Interest | TransferIn=> bal + tx.amount.value,
        Withdrawal | Expense | Buy | TransferOut=> bal - tx.amount.value,
    }
    ))
}

pub async fn net_worth(pool: &SqlitePool, provider: &yahoo::YahooConnector) -> Result<Decimal> {

    let accounts = get_all_accounts(pool).await?;

    let mut networth = Decimal::ZERO;
    for account in &accounts {
        println!("account name:{}", account.name);
        let balance = account_balance(pool, account.id).await?;
        networth += balance;
        println!("account cash balance:{}", balance);

        let holdings = account_holdings(pool, account.id).await?;
        for holding in &holdings{
            println!("holding {} of quantity {}",holding.ticker,holding.quantity);
            let (value,timestamp) = get_price_by_ticker(provider, holding.ticker.as_str()).await?; 
            println!("at {}, the price of {} was {}",timestamp, holding.ticker,value);
            let total_value = holding.quantity * value;
            println!("total value {}", total_value);
            networth+=total_value;
        }
    }
    println!("{}",networth);
    Ok(networth)
}