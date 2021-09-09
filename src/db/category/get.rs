//! SQL operations for getting category information.

use super::{
    super::error::Result,
    id::{InfoWithID, Locator},
    GcID,
};
use rusqlite::named_params;

use crate::model::{
    attempt,
    game::{
        category::{AttemptInfo, Info, ShortDescriptor},
        Split,
    },
    short,
};

/// Object for getting category information from the database.
pub struct Getter<'conn> {
    query_info_all: rusqlite::Statement<'conn>,
    query_attempt_info: rusqlite::Statement<'conn>,
    query_info_by_short: rusqlite::Statement<'conn>,
    query_info_by_id: rusqlite::Statement<'conn>,
    query_splits: rusqlite::Statement<'conn>,
}

impl<'conn> AsMut<Getter<'conn>> for Getter<'conn> {
    fn as_mut(&mut self) -> &mut Getter<'conn> {
        self
    }
}

impl<'conn> Getter<'conn> {
    pub(crate) fn new(conn: &'conn rusqlite::Connection) -> Result<Self> {
        Ok(Self {
            query_info_all: conn.prepare(SQL_INFO_ALL)?,
            query_info_by_short: conn.prepare(SQL_INFO_BY_SHORT)?,
            query_info_by_id: conn.prepare(SQL_INFO_BY_ID)?,
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
    pub fn init_session<'b>(&mut self, info: InfoWithID) -> Result<attempt::Session<'b>> {
        let attempt = self.attempt_info(&info)?;
        let splits = self.splits(&info)?;
        let run = attempt::Run {
            attempt,
            splits: attempt::split::Set::new(splits),
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
                let g_short: short::Name = row.get("gshort")?;
                let c_short: short::Name = row.get("cshort")?;
                Ok(Info {
                    game: row.get("gname")?,
                    category: row.get("cname")?,
                    short: ShortDescriptor::new(g_short, c_short),
                })
            })?
            .collect()
    }

    /// Resolves a short descriptor `short` to a category info record.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn info_from_short(&mut self, short: &ShortDescriptor) -> Result<InfoWithID> {
        Ok(self.query_info_by_short.query_row(
            named_params![":game": short.game, ":category": short.category],
            |row| {
                Ok(InfoWithID {
                    id: row.get("gcid")?,
                    info: Info {
                        game: row.get("gname")?,
                        category: row.get("cname")?,
                        short: short.clone(),
                    },
                })
            },
        )?)
    }

    /// Resolves a game-category ID `gcid` to a category info record.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn info_from_id(&mut self, gcid: GcID) -> Result<InfoWithID> {
        Ok(self
            .query_info_by_id
            .query_row(named_params![":game_category": gcid], |row| {
                Ok(InfoWithID {
                    id: gcid,
                    info: Info {
                        game: row.get("gname")?,
                        category: row.get("cname")?,
                        short: ShortDescriptor {
                            game: row.get("gshort")?,
                            category: row.get("cshort")?,
                        },
                    },
                })
            })?)
    }

    /// Gets attempt information for a game/category located by `locator`.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn attempt_info<L: Locator>(&mut self, locator: &L) -> Result<AttemptInfo> {
        let game_category = locator.locate_gcid(self)?;
        Ok(self.query_attempt_info.query_row(
            named_params![":game_category": game_category],
            |row| {
                Ok(AttemptInfo {
                    total: row.get("total")?,
                    completed: row.get("completed")?,
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
        let game_category = locator.locate_gcid(self)?;
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
 ORDER BY gshort ASC, cshort ASC;";

const SQL_INFO_BY_SHORT: &str = "
SELECT game_category_id AS gcid
     , game.name        AS gname
     , category.name    AS cname
  FROM game
       INNER JOIN game_category USING(game_id)
       INNER JOIN category      USING(category_id)
 WHERE game.short = :game AND category.short = :category;";

const SQL_INFO_BY_ID: &str = "
SELECT game.short     AS gshort
     , game.name      AS gname
     , category.short AS cshort
     , category.name  AS cname
  FROM game
       INNER JOIN game_category USING(game_id)
       INNER JOIN category      USING(category_id)
 WHERE game_category_id = :game_category;";

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
