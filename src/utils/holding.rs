use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::models::holding::Holding;
use uuid::Uuid;
use std::collections::BTreeMap;
use crate::models::TransactionKind::*;
use crate::queries::transaction::list_transactions;
use crate::error::{Error,Result};


pub async fn get_price_by_ticker(provider: &yahoo::YahooConnector, ticker: &str)-> Result<(Decimal,OffsetDateTime)>{
    let response = provider.get_latest_quotes(ticker, "1d").await?;
    let quote = response.last_quote()?;
    let time = OffsetDateTime::from_unix_timestamp(quote.timestamp)
    .map_err(|_| Error::NoPrice(ticker.into()))?;

    let price = Decimal::from_f64(quote.close)
    .ok_or_else(|| Error::NoPrice(ticker.into()))?;
    Ok((price, time))
}

pub async fn account_holdings<'e, E>(exec: E, account_id: Uuid) -> Result<Vec<Holding>>
where 
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>
{
    let mut txs = list_transactions(exec, account_id).await?;
    txs.reverse();

    //key is ticker, the value is the total holdings of that ticket
    //loop through all the transactions and calculate how many of whatever ticker fund is owned
    let mut map:BTreeMap<String, Holding> = BTreeMap::new();

    for tx in txs.iter(){
        let (Some(ticker), Some(qty)) = (&tx.ticker, tx.quantity) else { continue };
        let holding = map
                    .entry(ticker.to_string())
                    .or_insert_with(|| Holding::new(ticker.to_string(), 
                    Decimal::ZERO, Decimal::ZERO));
        match tx.kind {
            Buy=> {
                holding.apply_buy(qty, tx.amount);
            },
            Sell=>{
                holding.apply_sell(qty)?;
            }
            _ =>{},
        }
    }

    Ok(map
    .into_values()
    .filter(|h| h.quantity > Decimal::ZERO)
    .collect())
}