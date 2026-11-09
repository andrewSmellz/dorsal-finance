use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::SqlitePool;
use std::collections::BTreeMap;

use yahoo_finance_api as yahoo;
use crate::error::Result;

use crate::models::{Currency, TransactionKind::*};
use crate::queries::account::get_all_accounts;
use crate::queries::transaction::list_transactions;
use crate::utils::holding::{account_holdings, get_price_by_ticker};


pub async fn account_balance_by_currency<'e, E>(exec: E, account_id: Uuid) -> Result<BTreeMap<Currency, Decimal>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
    let txs = list_transactions(exec, account_id).await?;
    let mut balances: BTreeMap<Currency, Decimal> = BTreeMap::new();

    for tx in &txs{
        let entry = balances.entry(tx.amount.currency).or_insert(Decimal::ZERO);
        match tx.kind {
            Deposit | Income | Sell | Dividend | Interest | TransferIn => *entry += tx.amount.value,
            Withdrawal | Expense | Buy | TransferOut => *entry -= tx.amount.value,
        }
    }
    
    Ok(balances)
}

pub async fn account_balance<'e, E>(exec: E, account_id: Uuid, currency: Currency) -> Result<Decimal>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let balances = account_balance_by_currency(exec, account_id).await?;
    Ok(balances.get(&currency).copied().unwrap_or(Decimal::ZERO))
}


pub async fn get_fx_rate(provider: &yahoo::YahooConnector, from:Currency, to:Currency)-> Result<Decimal>{
    if from == to {
        return Ok(Decimal::ONE);
    }
    let fx_ticker = format!("{}{}=X", from, to);
    let (rate, _) = get_price_by_ticker(provider, &fx_ticker).await?;
    Ok(rate)
}

pub async fn net_worth(pool: &SqlitePool, provider: &yahoo::YahooConnector, base: Currency) -> Result<Decimal> {
    let accounts = get_all_accounts(pool).await?;

    let mut networth = Decimal::ZERO;
    for account in &accounts {
        println!("account name:{}", account.name);
        let balances = account_balance_by_currency(pool, account.id).await?;
        for (currency, amount) in &balances{
                let rate = get_fx_rate(provider, *currency, base).await?;
                let converted = amount * rate;
                println!("  cash {} {} -> {} {} (rate {})", amount, currency, converted, base, rate);
                networth += converted * rate;       
        }
        
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