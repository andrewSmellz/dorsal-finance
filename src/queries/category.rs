use crate::error::{Error, Result};
use crate::models::category::validate_name;
use crate::models::{Category, CategoryId};
use sqlx::SqlitePool;
use uuid::Uuid;

fn map_unique(e: sqlx::Error, name: &str) -> Error {
    match e {
        sqlx::Error::Database(ref d) if d.is_unique_violation() => {
            Error::DuplicateCategory(name.to_string())
        }
        other => other.into(),
    }
}

async fn insert_category<'e, E>(
    exec: E,
    parent_id: Option<CategoryId>,
    name: String,
) -> Result<Category>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let id = CategoryId(Uuid::new_v4());

    sqlx::query!(
        "INSERT INTO categories (id, parent_id, name) VALUES (?, ?, ?)",
        id,
        parent_id,
        name
    )
    .execute(exec)
    .await
    .map_err(|e| map_unique(e, &name))?;

    Ok(Category {
        id,
        parent_id,
        name,
        archived: false,
    })
}

/// Creates one category under `parent` (None = top level). The parent must exist and be active.
pub async fn create_category(
    pool: &SqlitePool,
    parent: Option<CategoryId>,
    name: &str,
) -> Result<Category> {
    let name = validate_name(name)?;
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    if let Some(p) = parent {
        ensure_category(&mut *tx, p, false).await?;
    }
    let category = insert_category(&mut *tx, parent, name).await?;

    tx.commit().await?;
    Ok(category)
}

/// Flat list of every category; build the tree (or paths) from `parent_id`.
pub async fn list_categories<'e, E>(exec: E, include_archived: bool) -> Result<Vec<Category>>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    Ok(sqlx::query_as!(
        Category,
        r#"SELECT id as "id!: CategoryId",
                  parent_id as "parent_id: CategoryId",
                  name as "name!",
                  archived as "archived!: bool"
           FROM categories
           WHERE (? OR archived = 0)
           ORDER BY name"#,
        include_archived
    )
    .fetch_all(exec)
    .await?)
}

/// Renames one level. Children keep pointing at it, so nothing else changes.
pub async fn rename_category(pool: &SqlitePool, id: CategoryId, new_name: &str) -> Result<()> {
    let name = validate_name(new_name)?;

    let res = sqlx::query!("UPDATE categories SET name = ? WHERE id = ?", name, id)
        .execute(pool)
        .await
        .map_err(|e| map_unique(e, &name))?;
    if res.rows_affected() == 0 {
        return Err(Error::CategoryNotFound(id.0));
    }
    Ok(())
}

/// Moves a category (and everything under it) to a new parent; None makes it top level.
pub async fn move_category(
    pool: &SqlitePool,
    id: CategoryId,
    new_parent: Option<CategoryId>,
) -> Result<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let name = sqlx::query_scalar!(r#"SELECT name as "name!" FROM categories WHERE id = ?"#, id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::CategoryNotFound(id.0))?;

    if let Some(p) = new_parent {
        ensure_category(&mut *tx, p, false).await?;

        // walk up from the new parent; hitting `id` means it would end up under itself
        let cycle = sqlx::query_scalar!(
            r#"WITH RECURSIVE ancestors(id, parent_id) AS (
                   SELECT id, parent_id FROM categories WHERE id = ?
                   UNION ALL
                   SELECT c.id, c.parent_id FROM categories c JOIN ancestors a ON c.id = a.parent_id
               )
               SELECT EXISTS(SELECT 1 FROM ancestors WHERE id = ?) as "cycle!: bool""#,
            p,
            id
        )
        .fetch_one(&mut *tx)
        .await?;
        if cycle {
            return Err(Error::CategoryCycle);
        }
    }

    sqlx::query!(
        "UPDATE categories SET parent_id = ? WHERE id = ?",
        new_parent,
        id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| map_unique(e, &name))?;

    tx.commit().await?;
    Ok(())
}

/// Archiving also archives everything under it. Unarchiving only restores this one,
/// and only if its parent is active.
pub async fn set_category_archived(
    pool: &SqlitePool,
    id: CategoryId,
    archived: bool,
) -> Result<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let row = sqlx::query!(
        r#"SELECT parent_id as "parent_id: CategoryId" FROM categories WHERE id = ?"#,
        id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::CategoryNotFound(id.0))?;

    if archived {
        sqlx::query!(
            r#"WITH RECURSIVE sub(id) AS (
                   SELECT id FROM categories WHERE id = ?
                   UNION ALL
                   SELECT c.id FROM categories c JOIN sub ON c.parent_id = sub.id
               )
               UPDATE categories SET archived = 1 WHERE id IN (SELECT id FROM sub)"#,
            id
        )
        .execute(&mut *tx)
        .await?;
    } else {
        if let Some(p) = row.parent_id {
            ensure_category(&mut *tx, p, false).await?;
        }
        sqlx::query!("UPDATE categories SET archived = 0 WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// Hard delete: only when no transaction uses it and it has no children.
pub async fn delete_category(pool: &SqlitePool, id: CategoryId) -> Result<()> {
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    let used = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM transactions WHERE category_id = ?) as "used!: bool""#,
        id
    )
    .fetch_one(&mut *tx)
    .await?;
    if used {
        return Err(Error::CategoryInUse(id.0));
    }

    let has_children = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM categories WHERE parent_id = ?) as "has_children!: bool""#,
        id
    )
    .fetch_one(&mut *tx)
    .await?;
    if has_children {
        return Err(Error::CategoryHasChildren(id.0));
    }

    let res = sqlx::query!("DELETE FROM categories WHERE id = ?", id)
        .execute(&mut *tx)
        .await?;
    if res.rows_affected() == 0 {
        return Err(Error::CategoryNotFound(id.0));
    }

    tx.commit().await?;
    Ok(())
}

/// Checked inside the transaction that records the expense/refund.
pub async fn ensure_category<'e, E>(exec: E, id: CategoryId, allow_archived: bool) -> Result<()>
where
    E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
{
    let archived = sqlx::query_scalar!(
        r#"SELECT archived as "archived!: bool" FROM categories WHERE id = ?"#,
        id
    )
    .fetch_optional(exec)
    .await?
    .ok_or(Error::CategoryNotFound(id.0))?;

    if archived && !allow_archived {
        return Err(Error::CategoryArchived(id.0));
    }
    Ok(())
}
