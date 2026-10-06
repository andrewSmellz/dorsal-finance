use crate::models;
use crate::Account;
use sqlx::SqlitePool;

pub async fn make_account(pool: &SqlitePool,account: Account) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO accounts (id, name, kind) VALUES (?, ?, ?)",
        account.id,
        account.name,
        account.kind
    )
    .execute(pool)
    .await?;

    Ok(())
}