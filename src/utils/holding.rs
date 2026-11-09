use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use crate::models::holding::Holding;
use uuid::Uuid;
use std::collections::BTreeMap;
use crate::models::TransactionKind::*;
use crate::models::{Money,Currency};
use crate::queries::transaction::list_transactions;
use crate::error::{Error,Result};


pub async fn get_price_by_ticker(provider: &yahoo::YahooConnector, ticker: &str)-> Result<(Money,OffsetDateTime)>{
    let response = provider.get_latest_quotes(ticker, "1d").await?;
    let quote = response.last_quote()?;
    let time = OffsetDateTime::from_unix_timestamp(quote.timestamp)
    .map_err(|_| Error::NoPrice(ticker.into()))?;

    let price = Decimal::from_f64(quote.close)
    .ok_or_else(|| Error::NoPrice(ticker.into()))?;

    let currency = response
    .metadata()?
    .currency
    .and_then(|c| c.parse::<Currency>().ok())
    .ok_or_else(|| Error::NoPrice(ticker.into()))?;

    Ok((Money::new(price, currency), time))
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

    for tx in &txs{
        let Some(position) = &tx.position else { continue };
        let holding = map
            .entry(position.ticker.clone())
            .or_insert_with(|| Holding::new(position.ticker.clone(), tx.amount.currency));
        match tx.kind {
            Buy=> {
                holding.apply_buy(position.quantity, tx.amount)?;
            },
            Sell=>{
                holding.apply_sell(position.quantity)?;
            }
            _ =>{},
        }
    }

    Ok(map
    .into_values()
    .filter(|h| h.position.quantity > Decimal::ZERO)
    .collect())
}