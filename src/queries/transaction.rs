use sqlx::SqlitePool;
use crate::{models::{CashKind,Transaction,Money,Position, TransactionKind::{self, Buy, Sell, TransferIn, TransferOut}}, utils::balance::account_balance};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use crate::error::{Error, Result};
use crate::models::currency::Currency;



async fn make_transaction<'e, E>(exec: E, tx: Transaction) -> Result<()>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let amount = tx.amount.value.to_string();
    let currency = tx.amount.currency.to_string();

    let ticker = tx.position.as_ref().map(|p| p.ticker.as_str());
    let quantity = tx.position.as_ref().map(|p| p.quantity.to_string());

    sqlx::query!("
        INSERT INTO transactions
     (id, account_id, kind, amount, ticker, quantity, timestamp, note, currency)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        tx.id, tx.account_id, tx.kind, amount, ticker, quantity, tx.timestamp, tx.note, currency
    )
    .execute(exec)
    .await?;
    Ok(())
}


fn parse_decimal(s: &str) -> Result<Decimal>{
    Ok(s.parse::<Decimal>()?)
}

fn parse_position(ticker: Option<String>, quantity: Option<&str>) -> Result<Option<Position>> {
    match (ticker, quantity) {
        (Some(ticker), Some(q)) => Ok(Some(Position::new(ticker,parse_decimal(q)?))),
        _ => Ok(None),
    }
}

pub async fn list_transactions<'e, E>(exec: E, account_id:Uuid) -> Result<Vec<Transaction>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let rows = sqlx::query!(
        r#"SELECT id as "id!: Uuid",
                  account_id as "account_id!: Uuid",
                  kind as "kind!: TransactionKind",
                  amount as "amount!",
                  currency as "currency!: Currency",
                  ticker,
                  quantity,
                  timestamp as "timestamp!: DateTime<Utc>",
                  note
           FROM transactions
           WHERE account_id = ?
           ORDER BY timestamp DESC"#,
        account_id
    )
    .fetch_all(exec)
    .await?;

    rows.into_iter().map(|t|{
        Ok(Transaction{
            id: t.id,
            account_id: t.account_id,
            kind: t.kind,
            amount: Money::new(parse_decimal(&t.amount)?, t.currency),
            position: parse_position(t.ticker, t.quantity.as_deref())?,
            timestamp: t.timestamp,
            note: t.note,
        })
        
    }).collect()
}

pub async fn list_all_transactions<'e, E>(exec: E) -> Result<Vec<Transaction>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let rows = sqlx::query!(
        r#"SELECT 
            id as "id!: Uuid",
            account_id as "account_id!: Uuid",
            kind as "kind!: TransactionKind",
            amount as "amount!",
            currency as "currency!: Currency",
            ticker,
            quantity,
            timestamp as "timestamp!: DateTime<Utc>",
            note
        FROM transactions
        ORDER BY timestamp DESC"#
    )
    .fetch_all(exec)
    .await?;

    rows.into_iter().map(|t|{
        Ok(Transaction{
            id: t.id,
            account_id: t.account_id,
            kind: t.kind,
            amount: Money::new(parse_decimal(&t.amount)?, t.currency),
            position: parse_position(t.ticker, t.quantity.as_deref())?,
            timestamp: t.timestamp,
            note: t.note,
        })
        
    }).collect()
}


pub async fn transfer(pool: &SqlitePool, from: Uuid, to: Uuid, amount:Money) -> Result<()>{
    if to == from {
        return Err(Error::SameAccount);
    }
    if amount.value <= Decimal::ZERO {
        return Err(Error::InvalidAmount);
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    if account_balance(&mut *tx, from,amount.currency).await? < amount.value{
        return Err(Error::InsufficientFunds);
    }

    let from_tx = Transaction::new(from, TransferOut, amount);
    let to_tx = Transaction::new(to,TransferIn, amount);

    make_transaction(&mut *tx, from_tx).await?;
    make_transaction(&mut *tx, to_tx).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn cash_flow(pool: &SqlitePool, account_id:Uuid, kind: CashKind, amount:Money, note: Option<&str>)
-> Result<()>
{
    if amount.value <= Decimal::ZERO{
        return Err(Error::InvalidAmount);
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    
    if kind.is_outflow() && account_balance(&mut *tx, account_id, amount.currency).await? < amount.value{
        return Err(Error::InsufficientFunds);
    }

    let mut t = Transaction::new(account_id, kind.into(), amount);
    if let Some(n) = note {
        t = t.with_note(n);
    }
    make_transaction(&mut *tx, t).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn buy(pool: &SqlitePool, account_id: Uuid, position: Position, total_cost: Money) -> Result<()> {
    if position.quantity <= Decimal::ZERO {
        return Err(Error::InvalidAmount);
    }
    if total_cost.value <= Decimal::ZERO {
        return Err(Error::InvalidAmount);
    }
 
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
 
    if account_balance(&mut *tx, account_id, total_cost.currency).await? < total_cost.value {
        return Err(Error::InsufficientFunds);
    }
 
    let t = Transaction::new(account_id, Buy, total_cost).with_position(position.ticker,position.quantity);
    make_transaction(&mut *tx, t).await?;
    tx.commit().await?;
    Ok(())
}

async fn held_quantity<'e, E>(exec: E, account_id: Uuid, ticker: &str) -> Result<Decimal>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let rows = sqlx::query!(
        r#"SELECT kind as "kind!: TransactionKind", quantity
           FROM transactions
           WHERE account_id = ? AND ticker = ? AND kind IN ('buy', 'sell')"#,
        account_id,
        ticker
    )
    .fetch_all(exec)
    .await?;

    let mut held = Decimal::ZERO;

    for row in rows.iter(){
        let quantity = parse_decimal(
        row.quantity
        .as_deref()
        .ok_or_else(|| sqlx::Error::Protocol("buy/sell row missing quantity".into()))?,
        )?;   

        match row.kind{
            Buy => held += quantity,
            Sell => held -= quantity,
            _ => {}
        } 
    }
    Ok(held)
}

pub async fn sell(pool: &SqlitePool, account_id: Uuid, position: Position, total_return: Money) -> Result<()> {
    if position.quantity <= Decimal::ZERO {
        return Err(Error::InvalidAmount);
    }
    if total_return.value <= Decimal::ZERO {
        return Err(Error::InvalidAmount);
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let held = held_quantity(&mut *tx, account_id, &position.ticker).await?;
    if  held < position.quantity {
        return Err(Error::InsufficientShares { ticker: position.ticker.into(), requested: position.quantity, held: held});
    }

    let t = Transaction::new(account_id, Sell, total_return).with_position(position.ticker, position.quantity);
    make_transaction(&mut *tx, t).await?;
    tx.commit().await?;
    Ok(())
}