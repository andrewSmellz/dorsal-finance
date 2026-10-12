use sqlx::SqlitePool;
use crate::models::{Transaction, TransactionKind};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};



pub async fn make_transaction(pool: &SqlitePool, tx: Transaction) -> Result<(), sqlx::Error> {
    let amount = tx.amount.to_string();
    let quantity= tx.quantity.map(|q|q.to_string());
    sqlx::query!("
        INSERT INTO transactions
     (id, account_id, kind, amount, ticker, quantity, timestamp, note)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        tx.id, tx.account_id, tx.kind, amount, tx.ticker, quantity, tx.timestamp, tx.note
    )
    .execute(pool)
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