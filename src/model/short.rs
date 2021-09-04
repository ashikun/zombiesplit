//! Type aliases for short names and associated types.

use std::{convert::Infallible, str::FromStr};

use bimap::BiHashMap;
use rusqlite::{types::FromSql, ToSql};
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Type alias for short names.
#[derive(
    Clone, Copy, Debug, Hash, DeserializeFromStr, SerializeDisplay, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Name(symbol::Symbol);

impl Default for Name {
    fn default() -> Self {
        Name(symbol::Symbol::from(""))
    }
}

impl FromStr for Name {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Name(symbol::Symbol::from(s)))
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<symbol::Symbol>> From<T> for Name {
    fn from(x: T) -> Name {
        Name(x.into())
    }
}

impl ToSql for Name {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Text(self.to_string()),
        ))
    }
}

impl FromSql for Name {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Ok(Name(symbol::Symbol::from(value.as_str()?)))
    }
}

/// Type alias for bidirectional maps from short names to items.
pub type Bimap<T> = BiHashMap<Name, T>;

/// Type alias for maps from short names to items.
pub type Map<T> = std::collections::HashMap<Name, T>;

/// Type alias for insertion-ordered maps from short names to items.
pub type LinkedMap<T> = linked_hash_map::LinkedHashMap<Name, T>;
