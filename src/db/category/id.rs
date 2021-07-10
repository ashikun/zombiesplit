//! Methods of identifying a game-category pair.

use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput},
    ToSql,
};

use super::{super::error::Result, Getter};
use crate::model::game::category::{Info, ShortDescriptor};

/// A game-category ID.
#[derive(Copy, Clone, Debug)]
pub struct GcID(pub i64);

impl ToSql for GcID {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl FromSql for GcID {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_i64().map(GcID)
    }
}

/// Trait for anything that can be used to find a game-category ID.
///
/// Some locators will result in database queries.
pub trait Locator {
    /// Locates the game-category ID using this locator, potentially using
    /// the given `getter` to resolve database queries.
    ///
    /// # Errors
    ///
    /// Typically, errors returned will be database errors.
    fn locate(&self, getter: &mut Getter) -> Result<GcID>;

    /// Tries to extract a game-category ID directly from this locator.
    fn as_game_category_id(&self) -> Option<GcID> {
        None
    }
}

/// Signed 64-bit integers are treated as game-category IDs natively.
impl Locator for GcID {
    fn locate(&self, _: &mut Getter) -> Result<GcID> {
        Ok(*self)
    }

    fn as_game_category_id(&self) -> Option<GcID> {
        Some(*self)
    }
}

/// An information record with an attached game-category ID.
pub struct InfoWithID {
    pub id: GcID,
    pub info: Info,
}

/// Category info implicitly contains a game-category ID.
impl Locator for InfoWithID {
    fn locate(&self, getter: &mut Getter) -> Result<GcID> {
        self.id.locate(getter)
    }

    fn as_game_category_id(&self) -> Option<GcID> {
        self.id.as_game_category_id()
    }
}

impl Locator for ShortDescriptor {
    fn locate(&self, getter: &mut Getter) -> Result<GcID> {
        // TODO(@MattWindsor91): make this a bit more optimal?
        Ok(getter.game_category_info(self)?.id)
    }
}
