use sqlx::SqlitePool;
use crate::{models::{Transaction, TransactionKind::{self, TransferIn, TransferOut}, transaction}, queries::account::get_account_by_id, utils::balance::account_balance};
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

pub async fn transfer(pool: &SqlitePool, from: Uuid, to: Uuid, amount:Decimal) -> Result<(), sqlx::Error>{
    if to==from{
        return Err(sqlx::Error::Protocol("cannot transfer into same account".into()));
    }
    if amount<=Decimal::ZERO{
        return Err(sqlx::Error::Protocol("amount must be greater than zero".into()));
    }
    if account_balance(pool, from).await < amount{
        return Err(sqlx::Error::Protocol("insufficient account balance".into()));   
    }

    let mut tx: sqlx::Transaction<'_, sqlx::Sqlite> = pool.begin().await?;

    let from_tx = Transaction::new(from, TransferOut, amount);
    let to_tx = Transaction::new(to,TransferIn, amount);

    make_transaction(&mut *tx, from_tx).await?;
    make_transaction(&mut *tx, to_tx).await?;

    tx.commit().await?;
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