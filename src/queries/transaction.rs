use sqlx::{SqlitePool, query};
use crate::{models::{CashKind, Transaction, TransactionKind::{self, Buy, Sell, TransferIn, TransferOut}, transaction}, queries::account::get_account_by_id, utils::balance::account_balance};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};



pub async fn make_transaction<'e, E>(exec: E, tx: Transaction) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let amount = tx.amount.to_string();
    let quantity= tx.quantity.map(|q|q.to_string());
    sqlx::query!("
        INSERT INTO transactions
     (id, account_id, kind, amount, ticker, quantity, timestamp, note)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        tx.id, tx.account_id, tx.kind, amount, tx.ticker, quantity, tx.timestamp, tx.note
    )
    .execute(exec)
    .await?;
    Ok(())
}


fn parse_decimal(s: &str) -> Result<Decimal,sqlx::Error>{
    s.parse::<Decimal>().map_err(|e|sqlx::Error::Decode(Box::new(e)))
}

pub async fn list_transactions(pool: &SqlitePool, account_id:Uuid) -> Result<Vec<Transaction>, sqlx::Error>{
    let rows = sqlx::query!(
        r#"SELECT id as "id!: Uuid",
                  account_id as "account_id!: Uuid",
                  kind as "kind!: TransactionKind",
                  amount as "amount!",
                  ticker,
                  quantity,
                  timestamp as "timestamp!: DateTime<Utc>",
                  note
           FROM transactions
           WHERE account_id = ?
           ORDER BY timestamp DESC"#,
        account_id
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(|t|{
        Ok(Transaction{
            id: t.id,
            account_id: t.account_id,
            kind: t.kind,
                amount: parse_decimal(&t.amount)?,
                ticker: t.ticker,
                quantity: t.quantity.as_deref().map(parse_decimal).transpose()?,
                timestamp: t.timestamp,
                note: t.note,
        })
        
    }).collect()
}

pub async fn transfer(pool: &SqlitePool, from: Uuid, to: Uuid, amount:Decimal) -> Result<(), sqlx::Error>{
    if to==from{
        return Err(sqlx::Error::Protocol("cannot transfer into same account".into()));
    }
    if amount<=Decimal::ZERO{
        return Err(sqlx::Error::Protocol("amount must be greater than zero".into()));
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    if account_balance(pool, from).await < amount{
        return Err(sqlx::Error::Protocol("insufficient account balance".into()));   
    }

    let from_tx = Transaction::new(from, TransferOut, amount);
    let to_tx = Transaction::new(to,TransferIn, amount);

    make_transaction(&mut *tx, from_tx).await?;
    make_transaction(&mut *tx, to_tx).await?;

    tx.commit().await
}

pub async fn cash_flow(pool: &SqlitePool, account_id:Uuid, kind: CashKind, amount:Decimal, note: Option<&str>)
-> Result<(), sqlx::Error>
{
    if amount < Decimal::ZERO{
        return Err(sqlx::Error::Protocol("amount must be greater than zero".into()));
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
    
    if kind.is_outflow() && account_balance(pool, account_id).await < amount{
        return Err(sqlx::Error::Protocol("insufficient account balance".into()));
    }

    let mut t = Transaction::new(account_id, kind.into(), amount);
    if let Some(n) = note {
        t = t.with_note(n);
    }
    make_transaction(&mut *tx, t).await?;
    tx.commit().await
}

pub async fn buy(pool: &SqlitePool, account_id: Uuid, ticker: &str, quantity: Decimal, total_cost: Decimal) -> Result<(), sqlx::Error> {
    if quantity <= Decimal::ZERO {
        return Err(sqlx::Error::Protocol("invalid quantity".into()));
    }
    if total_cost <= Decimal::ZERO {
        return Err(sqlx::Error::Protocol("invalid cost".into()));
    }
 
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
 
    if account_balance(pool, account_id).await < total_cost {
        return Err(sqlx::Error::Protocol("insufficient account balance".into()));
    }
 
    let t = Transaction::new(account_id, Buy, total_cost).with_ticker(ticker).with_quantity(quantity);
    make_transaction(&mut *tx, t).await?;
    tx.commit().await
}

async fn held_quantity<'e, E>(exec: E, account_id: Uuid, ticker: &str) -> Result<Decimal, sqlx::Error>
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

pub async fn sell(pool: &SqlitePool, account_id: Uuid, ticker: &str, quantity: Decimal, total_return: Decimal) -> Result<(), sqlx::Error> {
    if quantity <= Decimal::ZERO {
        return Err(sqlx::Error::Protocol("invalid quantity".into()));
    }
    if total_return <= Decimal::ZERO {
        return Err(sqlx::Error::Protocol("invalid returns".into()));
    }

    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    if held_quantity(&mut *tx, account_id, ticker).await? < quantity {
        return Err(sqlx::Error::Protocol("insufficient shares held".into()));
    }

    let t = Transaction::new(account_id, Sell, total_return).with_ticker(ticker).with_quantity(quantity);
    make_transaction(&mut *tx, t).await?;
    tx.commit().await
}

