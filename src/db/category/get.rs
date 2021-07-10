//! SQL operations for getting category information.

use super::{
    super::error::Result,
    id::{InfoWithID, Locator},
};
use rusqlite::named_params;

use crate::model::{
    attempt,
    game::{
        category::{AttemptInfo, Info, ShortDescriptor},
        Split,
    },
};

/// Object for getting category information from the database.
pub struct Getter<'conn> {
    query_info_all: rusqlite::Statement<'conn>,
    query_attempt_info: rusqlite::Statement<'conn>,
    query_info_by_short: rusqlite::Statement<'conn>,
    query_splits: rusqlite::Statement<'conn>,
}

impl<'conn> Getter<'conn> {
    pub(crate) fn new(conn: &'conn rusqlite::Connection) -> Result<Self> {
        Ok(Self {
            query_info_all: conn.prepare(SQL_INFO_ALL)?,
            query_info_by_short: conn.prepare(SQL_INFO_BY_SHORT)?,
            query_attempt_info: conn.prepare(SQL_ATTEMPT_INFO)?,
            query_splits: conn.prepare(SQL_SPLITS)?,
        })
    }

    /// Initialises an attempt session for the game/category referred to by
    /// `desc`.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn init_session(&mut self, desc: &ShortDescriptor) -> Result<attempt::Session> {
        let info = self.game_category_info(&desc)?;
        let attempt_info = self.attempt_info(&info)?;
        let splits = self.splits(&info)?;
        // TODO(@MattWindsor91): track attempts properly.
        let run = attempt::Run {
            attempt: attempt_info.total + 1,
            splits: splits.into_iter().map(attempt::Split::new).collect(),
        };
        Ok(attempt::Session::new(info.info, run))
    }

    /// Gets information records for all game-category pairs on the database.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn all_game_category_info(&mut self) -> Result<Vec<Info>> {
        self.query_info_all
            .query_and_then([], |row| {
                let g_short: String = row.get("gshort")?;
                let c_short: String = row.get("cshort")?;
                Ok(Info {
                    game: row.get("gname")?,
                    category: row.get("cname")?,
                    short: ShortDescriptor::new(&g_short, &c_short),
                })
            })?
            .collect()
    }

    /// Resolves a short descriptor `desc` to a category info record.
    ///
    /// The info record can then be used to query other things from the
    /// category database, as it implements [Locator].
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn game_category_info(&mut self, short: &ShortDescriptor) -> Result<InfoWithID> {
        Ok(self.query_info_by_short.query_row(
            named_params![":game": short.game, ":category": short.category],
            |row| {
                Ok(InfoWithID {
                    id: row.get(0)?,
                    info: Info {
                        game: row.get(1)?,
                        category: row.get(2)?,
                        short: short.clone(),
                    },
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
                    short: row.get(1)?,
                    name: row.get(2)?,
                })
            })?
            .collect()
    }
}

const SQL_INFO_ALL: &str = "
    SELECT game.short     AS gshort
         , game.name      AS gname
         , category.short AS cshort
         , category.name  AS cname
    FROM game
        INNER JOIN game_category USING(game_id)
        INNER JOIN category      USING(category_id)
    ORDER BY gshort ASC, cshort ASC
    ;";

const SQL_INFO_BY_SHORT: &str = "
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
    SELECT split_id, split.short, split.name
    FROM split
            INNER JOIN segment_split    USING(split_id)
            INNER JOIN category_segment USING(segment_id)
            INNER JOIN game_category    USING(category_id)
    WHERE game_category_id = :game_category
    ORDER BY category_segment.position ASC
            , segment_split.position   ASC
    ;";
