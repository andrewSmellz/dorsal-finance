use uuid::Uuid;
use crate::error::{Error, Result};

/// Only used to write and display paths like "Food/Groceries". Names themselves can't contain it.
pub const CATEGORY_DELIMITER: char = '/';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct CategoryId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: CategoryId,
    /// None for top-level categories
    pub parent_id: Option<CategoryId>,
    /// this level only, e.g. "Groceries", not "Food/Groceries"
    pub name: String,
    pub archived: bool,
}

/// One level of a category name: trimmed, non-empty, no delimiter.
pub fn validate_name(raw: &str) -> Result<String> {
    let name = raw.trim();
    if name.is_empty() || name.contains(CATEGORY_DELIMITER) {
        return Err(Error::InvalidCategoryName(raw.to_string()));
    }
    Ok(name.to_string())
}


/// Builds "Food/Groceries" by walking up the parent chain.
/// Pass the full list (including archived), or an archived ancestor makes this return None.
pub fn category_path(all: &[Category], id: CategoryId) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    let mut current = Some(id);
    while let Some(cid) = current {
        if parts.len() > all.len() {
            return None; // would only happen with a cycle, which the queries prevent
        }
        let c = all.iter().find(|c| c.id == cid)?;
        parts.push(&c.name);
        current = c.parent_id;
    }
    parts.reverse();
    Some(parts.join(&CATEGORY_DELIMITER.to_string()))
}