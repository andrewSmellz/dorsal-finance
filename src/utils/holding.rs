use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::models::holding::Holding;
use crate::models::transaction::Transaction;
use uuid::Uuid;
use std::collections::BTreeMap;
use rust_decimal_macros::dec;
use crate::models::TransactionKind::*;
use crate::queries::transaction::list_transactions;

pub async fn get_price_by_ticker(provider: &yahoo::YahooConnector, ticker: &str)-> (Option<Decimal>,OffsetDateTime){
    let response = provider.get_latest_quotes(ticker, "1d").await.unwrap();
    let quote = response.last_quote().unwrap();
    let time = OffsetDateTime::from_unix_timestamp(quote.timestamp).unwrap();
    println!("At {} the price of XEQT was {}", time, quote.close);
    (Decimal::from_f64(quote.close),time)
}

pub async fn account_holdings<'e, E>(exec: E, account_id: Uuid) -> Result<Vec<Holding>, sqlx::Error>
where 
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>
{
    let mut txs = list_transactions(exec, account_id).await?;
    txs.reverse();

    //key is ticker, the value is the total holdings of that ticket
    //loop through all the transactions and calculate how many of whatever ticker fund is owned
    let mut map:BTreeMap<String, Holding> = BTreeMap::new();

    for tx in txs.iter(){
        let holding = map
                    .entry(tx.ticker.clone().unwrap())
                    .or_insert_with(|| Holding::new(tx.ticker.clone().unwrap(), 
                    Decimal::ZERO, Decimal::ZERO));
        match tx.kind {
            Buy=> {

                holding.apply_buy(tx.quantity.unwrap(), tx.amount);
            },
            Sell=>{
                holding.apply_sell(tx.quantity.unwrap());

            }
            _ =>{},
        }
    }

    Ok(map
    .into_values()
    .filter(|h| h.quantity > Decimal::ZERO)
    .collect())
}