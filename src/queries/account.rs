use crate::models::{Account, AccountKind};
use sqlx::SqlitePool;
use uuid::Uuid;

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

pub async fn get_account(pool: &SqlitePool, account_name: String) -> Result<Option<Account>, sqlx::Error>{
    sqlx::query_as!(
    Account,
    r#"SELECT id as "id!: Uuid",
     name as "name!",
     kind as "kind!: AccountKind"
     FROM accounts WHERE name = ?"#,
    account_name)
.fetch_optional(pool)
.await
}

pub async fn get_all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, sqlx::Error>{
    sqlx::query_as!(
        Account,
        r#"SELECT id as "id!: Uuid",
            name as "name!",
            kind as "kind!: AccountKind"
            FROM accounts
        "#
    ).fetch_all(pool)
    .await
}

/*
 * QUERIES SHOULD BE DONE VIA ID AND NOT ACCOUNT NAME
 * TODO make sure thats the 
 */