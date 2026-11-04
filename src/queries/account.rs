use crate::models::{Account, AccountKind};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::error::{Error, Result};

pub async fn make_account(pool: &SqlitePool,account: Account) -> Result<()> {
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

 //QUERIES SHOULD BE DONE VIA ID AND NOT ACCOUNT NAME
pub async fn get_account_by_name(pool: &SqlitePool, account_name: String) -> Result<Option<Account>>{
    Ok(sqlx::query_as!(
    Account,
    r#"SELECT id as "id!: Uuid",
     name as "name!",
     kind as "kind!: AccountKind"
     FROM accounts WHERE name = ?"#,
    account_name)
.fetch_optional(pool)
.await?)
}

 //QUERIES SHOULD BE DONE VIA ID AND NOT ACCOUNT NAME
pub async fn get_account_by_id(pool: &SqlitePool, account_id: Uuid) -> Result<Option<Account>>{
    Ok(sqlx::query_as!(
    Account,
    r#"SELECT id as "id!: Uuid",
     name as "name!",
     kind as "kind!: AccountKind"
     FROM accounts WHERE id = ?"#,
    account_id)
.fetch_optional(pool)
.await?)
}

pub async fn get_all_accounts<'e, E>(exec: E) -> Result<Vec<Account>>
where
E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    Ok(sqlx::query_as!(
        Account,
        r#"SELECT id as "id!: Uuid",
            name as "name!",
            kind as "kind!: AccountKind"
            FROM accounts
        "#
    ).fetch_all(exec)
    .await?)
}

