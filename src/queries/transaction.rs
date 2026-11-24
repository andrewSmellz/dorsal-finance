use sqlx::SqlitePool;
use crate::{models::{CashKind,CategoryId,Transaction,Money,Position, TransactionKind::{self, Buy, Sell, TransferIn, TransferOut}}, utils::balance::account_balance};
use crate::queries::category::ensure_category;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, NaiveDate, Utc};
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
     (id, transfer_id, account_id, kind, category_id, amount, currency, ticker, quantity, occurred_on, created_at, note)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        tx.id, tx.transfer_id, tx.account_id, tx.kind, tx.category, amount, currency,
        ticker, quantity, tx.occurred_on, tx.created_at, tx.note
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
                  transfer_id as "transfer_id: Uuid",
                  account_id as "account_id!: Uuid",
                  kind as "kind!: TransactionKind",
                  category_id as "category_id: CategoryId",
                  amount as "amount!",
                  currency as "currency!: Currency",
                  ticker,
                  quantity,
                  occurred_on as "occurred_on!: NaiveDate",
                  created_at as "created_at!: DateTime<Utc>",
                  note
           FROM transactions
           WHERE account_id = ?
           ORDER BY occurred_on DESC, created_at DESC"#,
        account_id
    )
    .fetch_all(exec)
    .await?;

    rows.into_iter().map(|t|{
        Ok(Transaction{
            id: t.id,
            transfer_id: t.transfer_id,
            account_id: t.account_id,
            kind: t.kind,
            category: t.category_id,
            amount: Money::new(parse_decimal(&t.amount)?, t.currency),
            position: parse_position(t.ticker, t.quantity.as_deref())?,
            occurred_on: t.occurred_on,
            created_at: t.created_at,
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
            transfer_id as "transfer_id: Uuid",
            account_id as "account_id!: Uuid",
            kind as "kind!: TransactionKind",
            category_id as "category_id: CategoryId",
            amount as "amount!",
            currency as "currency!: Currency",
            ticker,
            quantity,
            occurred_on as "occurred_on!: NaiveDate",
            created_at as "created_at!: DateTime<Utc>",
            note
        FROM transactions
        ORDER BY occurred_on DESC, created_at DESC"#
    )
    .fetch_all(exec)
    .await?;

    rows.into_iter().map(|t|{
        Ok(Transaction{
            id: t.id,
            transfer_id: t.transfer_id,
            account_id: t.account_id,
            kind: t.kind,
            category: t.category_id,
            amount: Money::new(parse_decimal(&t.amount)?, t.currency),
            position: parse_position(t.ticker, t.quantity.as_deref())?,
            occurred_on: t.occurred_on,
            created_at: t.created_at,
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

    let transfer_id = Uuid::new_v4();
    let from_tx = Transaction::new(from, TransferOut, amount).with_transfer_id(transfer_id);
    let to_tx = Transaction::new(to, TransferIn, amount).with_transfer_id(transfer_id);

    make_transaction(&mut *tx, from_tx).await?;
    make_transaction(&mut *tx, to_tx).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn cash_flow(pool: &SqlitePool, account_id:Uuid, kind: CashKind, category: Option<CategoryId>, amount:Money, note: Option<&str>)
-> Result<()>
{
    if amount.value <= Decimal::ZERO{
        return Err(Error::InvalidAmount);
    }

    match (kind.takes_category(), category) {
        (true, None) => return Err(Error::MissingCategory),
        (false, Some(_)) => return Err(Error::UnexpectedCategory),
        _ => {}
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    if let Some(c) = category {
        // a refund may point at a category that has since been archived
        ensure_category(&mut *tx, c, matches!(kind, CashKind::Refund)).await?;
    }

    if kind.is_outflow() && account_balance(&mut *tx, account_id, amount.currency).await? < amount.value{
        return Err(Error::InsufficientFunds);
    }

    let mut t = Transaction::new(account_id, kind.into(), amount);
    if let Some(c) = category {
        t = t.with_category(c);
    }
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

        match row.kind {
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