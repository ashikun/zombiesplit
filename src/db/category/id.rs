//! Methods of identifying a game-category pair.

use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput},
    ToSql,
};

use super::{super::error::Result, Getter};
use crate::model::game::category::{Info, ShortDescriptor};

/// A game-category ID.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    /// Locates the game-category using this locator, potentially using
    /// the given `getter` to resolve database queries.
    ///
    /// # Errors
    ///
    /// Typically, errors returned will be database errors.
    fn locate<'a>(&self, getter: &mut impl AsMut<Getter<'a>>) -> Result<InfoWithID>;

    /// Locates the game-category ID using this locator, potentially using
    /// the given `getter` to resolve database queries.
    ///
    /// # Errors
    ///
    /// Typically, errors returned will be database errors.
    fn locate_gcid<'a>(&self, getter: &mut impl AsMut<Getter<'a>>) -> Result<GcID> {
        self.locate(getter).map(|x| x.id)
    }

    /// Tries to extract a game-category ID directly from this locator.
    fn as_game_category_id(&self) -> Option<GcID> {
        None
    }
}

/// Signed 64-bit integers are treated as game-category IDs natively.
impl Locator for GcID {
    fn locate<'a>(&self, getter: &mut impl AsMut<Getter<'a>>) -> Result<InfoWithID> {
        getter.as_mut().info_from_id(*self)
    }

    fn locate_gcid<'a>(&self, _: &mut impl AsMut<Getter<'a>>) -> Result<GcID> {
        Ok(*self)
    }

    fn as_game_category_id(&self) -> Option<GcID> {
        Some(*self)
    }
}

/// An information record with an attached game-category ID.
#[derive(Debug, Clone)]
pub struct InfoWithID {
    pub id: GcID,
    pub info: Info,
}

/// Category info implicitly contains a game-category ID.
impl Locator for InfoWithID {
    fn locate<'a>(&self, _: &mut impl AsMut<Getter<'a>>) -> Result<InfoWithID> {
        Ok(self.clone())
    }

    fn locate_gcid<'a>(&self, _: &mut impl AsMut<Getter<'a>>) -> Result<GcID> {
        Ok(self.id)
    }

    fn as_game_category_id(&self) -> Option<GcID> {
        self.id.as_game_category_id()
    }
}

impl Locator for ShortDescriptor {
    fn locate<'a>(&self, getter: &mut impl AsMut<Getter<'a>>) -> Result<InfoWithID> {
        getter.as_mut().info_from_short(self)
    }
}
