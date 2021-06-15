//! Top-level module for the model's sqlite database.

pub mod error;
pub mod game;
mod init;
use crate::model::{
    game::Config,
    split::{self, Split},
    Metadata, Run, Session,
};
use rusqlite::params;
use std::path::Path;

pub use error::{Error, Result};

/// A connection to zombiesplit's database.
pub struct Db {
    pub(super) conn: rusqlite::Connection,
}

impl Db {
    /// Opens a database connection to a given file.
    ///
    /// # Errors
    ///
    /// Returns errors from the underlying database library if the connection
    /// opening failed.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self::from_sqlite(rusqlite::Connection::open(path)?))
    }

    /// Opens a database connection in memory.
    ///
    /// Useful for testing, mainly.
    ///
    /// # Errors
    ///
    /// Returns errors from the underlying database library if the connection
    /// opening failed.
    pub fn in_memory() -> Result<Self> {
        Ok(Self::from_sqlite(rusqlite::Connection::open_in_memory()?))
    }

    fn from_sqlite(conn: rusqlite::Connection) -> Self {
        Self { conn }
    }

    /// Initialises the database for first use.
    ///
    /// # Errors
    ///
    /// Propagates errors from the database if anything goes wrong.
    pub fn init(&self) -> Result<()> {
        for (name, ddl) in &[
            ("game", init::GAME_SQL),
            ("category", init::CATEGORY_SQL),
            ("game_category", init::GAME_CATEGORY_SQL),
            ("segment", init::SEGMENT_SQL),
            ("category_segment", init::CATEGORY_SEGMENT_SQL),
            ("split", init::SPLIT_SQL),
            ("segment_split", init::SEGMENT_SPLIT_SQL),
            ("run", init::RUN_SQL),
            ("run_split", init::RUN_SPLIT_SQL),
        ] {
            log::info!("creating table {}", name);
            let _ = self.conn.execute(ddl, [])?;
        }
        Ok(())
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&self, short: &str, game: &Config) -> Result<()> {
        game::Inserter::new(self)?.add_game(short, game)
    }

    /// Initialises a session for the short-named `game` and `category`.
    ///
    /// # Errors
    ///
    /// Fails if there is no such category for the given game in the database,
    /// or the game doesn't exist, or any other database error occurs.
    pub fn init_session(&self, game: &str, category: &str) -> Result<Session> {
        let metadata = self.get_metadata(game, category)?;
        let run = self.init_run(metadata.category_id)?;
        Ok(Session::new(metadata, run))
    }

    fn get_metadata(&self, game: &str, cat: &str) -> Result<Metadata> {
        self.conn.query_row_and_then(
            "
            SELECT category.id, game.name, category.name
            FROM       game
            INNER JOIN game_category ON (game.id                  = game_category.gameid)
            INNER JOIN category      ON (game_category.categoryid = category.id         )
            WHERE game.short     = ?1
              AND category.short = ?2
        ;",
            params![game, cat],
            |row| {
                Ok(Metadata {
                    category_id: row.get(0)?,
                    game: row.get(1)?,
                    category: row.get(2)?,
                })
            },
        )
    }

    fn init_run(&self, category_id: i64) -> Result<Run> {
        let attempt = self.conn.query_row(
            "
        SELECT count(id)
        FROM run
        WHERE categoryid = ?1
        ;",
            params![category_id],
            |row| row.get(0),
        )?;
        let splits = self.init_splits(category_id)?;
        Ok(Run { attempt, splits })
    }

    fn init_splits(&self, category_id: i64) -> Result<Vec<Split>> {
        // TODO(@MattWindsor91): get the segments too.
        self.conn
            .prepare(
                "
        SELECT split.id, split.name
        FROM       split
        INNER JOIN segment_split    ON (split.id                = segment_split.splitid     )
        INNER JOIN category_segment ON (segment_split.segmentid = category_segment.segmentid)
        WHERE category_segment.categoryid = ?1
        ORDER BY category_segment.position ASC
               , segment_split.position    ASC
        ;",
            )?
            .query_and_then(params![category_id], |row| {
                let meta = split::Metadata {
                    id: row.get(0)?,
                    name: row.get(1)?,
                };
                Ok(Split::new(meta))
            })?
            .collect()
    }
}
