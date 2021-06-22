//! Module for database activities related to storing and querying categories.

use rusqlite::named_params;

use super::error::Result;
use crate::model::{
    attempt,
    game::{
        category::{AttemptInfo, Info, ShortDescriptor},
        Split,
    },
    Session,
};

/// Object for getting category information from the database.
pub struct Getter<'conn> {
    query_attempt_info: rusqlite::Statement<'conn>,
    query_game_category_info: rusqlite::Statement<'conn>,
    query_splits: rusqlite::Statement<'conn>,
}

impl<'conn> Getter<'conn> {
    pub(super) fn new(conn: &'conn rusqlite::Connection) -> Result<Self> {
        Ok(Self {
            query_attempt_info: conn.prepare(SQL_ATTEMPT_INFO)?,
            query_game_category_info: conn.prepare(SQL_GAME_CATEGORY_INFO)?,
            query_splits: conn.prepare(SQL_SPLITS)?,
        })
    }

    /// Initialises an attempt session for the game/category referred to by
    /// `desc`.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn init_session(&mut self, desc: &ShortDescriptor) -> Result<Session> {
        let info = self.game_category_info(&desc)?;
        let attempt_info = self.attempt_info(&info)?;
        let splits = self.splits(&info)?;
        // TODO(@MattWindsor91): track attempts properly.
        let run = attempt::Run {
            attempt: attempt_info.total + 1,
            splits: splits.into_iter().map(attempt::Split::new).collect(),
        };
        Ok(Session::new(info, run))
    }

    /// Resolves a short descriptor `desc` to a category info record.
    ///
    /// The info record can then be used to query other things from the
    /// category database, as it implements [Locator].
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn game_category_info(&mut self, desc: &ShortDescriptor) -> Result<Info> {
        Ok(self.query_game_category_info.query_row(
            named_params![":game": desc.game, ":category": desc.category],
            |row| {
                Ok(Info {
                    game_category_id: row.get(0)?,
                    game: row.get(1)?,
                    category: row.get(2)?,
                })
            },
        )?)
    }

    /// Gets attempt information for a game/category located by `locator`.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn attempt_info<L: Locator>(&mut self, locator: &L) -> Result<AttemptInfo> {
        let game_category = locator.locate(self)?;
        Ok(self.query_attempt_info.query_row(
            named_params![":game_category": game_category],
            |row| {
                Ok(AttemptInfo {
                    total: row.get(0)?,
                    completed: row.get(1)?,
                })
            },
        )?)
    }

    /// Gets split information for a game/category located by `locator`.
    ///
    /// Splits are currently returned as a flat vector.  This may change
    /// eventually, to support segment display.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn splits<L: Locator>(&mut self, locator: &L) -> Result<Vec<Split>> {
        let game_category = locator.locate(self)?;
        // TODO(@MattWindsor91): get the segments too.
        self.query_splits
            .query_and_then(named_params![":game_category": game_category], |row| {
                Ok(Split {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect()
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
    fn locate(&self, getter: &mut Getter) -> Result<i64>;

    /// Tries to extract a game-category ID directly from this locator.
    fn as_game_category_id(&self) -> Option<i64> {
        None
    }
}

/// Signed 64-bit integers are treated as game-category IDs natively.
impl Locator for i64 {
    fn locate(&self, _: &mut Getter) -> Result<i64> {
        Ok(*self)
    }

    fn as_game_category_id(&self) -> Option<i64> {
        Some(*self)
    }
}

/// Category info implicitly contains a game-category ID.
impl Locator for Info {
    fn locate(&self, getter: &mut Getter) -> Result<i64> {
        self.game_category_id.locate(getter)
    }

    fn as_game_category_id(&self) -> Option<i64> {
        self.game_category_id.as_game_category_id()
    }
}

impl Locator for ShortDescriptor {
    fn locate(&self, getter: &mut Getter) -> Result<i64> {
        // TODO(@MattWindsor91): make this a bit more optimal?
        Ok(getter.game_category_info(self)?.game_category_id)
    }
}

const SQL_GAME_CATEGORY_INFO: &str = "
    SELECT game_category_id, game.name, category.name
    FROM game
        INNER JOIN game_category USING(game_id)
        INNER JOIN category      USING(category_id)
    WHERE game.short = :game AND category.short = :category;";

const SQL_ATTEMPT_INFO: &str = "
    SELECT COUNT(*)                     AS total,
           IFNULL(SUM(is_completed), 0) AS completed
    FROM run
    WHERE game_category_id = :game_category;";

const SQL_SPLITS: &str = "
    SELECT split_id, split.name
    FROM split
            INNER JOIN segment_split    USING(split_id)
            INNER JOIN category_segment USING(segment_id)
            INNER JOIN game_category    USING(category_id)
    WHERE game_category_id = :game_category
    ORDER BY category_segment.position ASC
            , segment_split.position   ASC
    ;";
