//! Top-level module for the model's sqlite database.

pub mod category;
pub mod error;
mod game;
mod init;
mod run;
use crate::model::{
    game::{category::ShortDescriptor, Config},
    history, Session,
};
use std::path::Path;

pub use error::{Error, Result};

use self::category::{GcID, Locator};

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
        init::on_db(&self.conn)
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&mut self, short: &str, game: &Config) -> Result<()> {
        let tx = self.conn.transaction()?;
        game::Inserter::new(&tx)?.add_game(short, game)?;
        Ok(tx.commit()?)
    }

    /// Adds the historic run `run` to the database.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a run
    /// fail.
    pub fn add_run<L: Locator>(&mut self, run: &history::TimedRun<L>) -> Result<()> {
        let run = run.with_locator(self.resolve_gcid(&run.category_locator)?);

        let tx = self.conn.transaction()?;
        run::Inserter::new(&tx)?.add(&run)?;
        Ok(tx.commit()?)
    }

    /// Gets summaries for the runs attached to the game-category located by
    /// `loc`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a run
    /// fail.
    pub fn runs_for<L: Locator>(&self, loc: &L) -> Result<Vec<history::RunSummary<GcID>>> {
        let id = self.resolve_gcid(loc)?;
        run::Finder::new(&self.conn)?.runs_for(id)
    }

    /// Initialises a session for the game/category described by the given
    /// short descriptor.
    ///
    /// # Errors
    ///
    /// Fails if there is no such category for the given game in the database,
    /// or the game doesn't exist, or any other database error occurs.
    pub fn init_session(&self, desc: &ShortDescriptor) -> Result<Session> {
        self.category_getter()?.init_session(&desc)
    }

    fn resolve_gcid<L: Locator>(&self, loc: &L) -> Result<GcID> {
        // TODO(@MattWindsor91): this is horrible.
        if let Some(x) = loc.as_game_category_id() {
            Ok(x)
        } else {
            loc.locate(&mut self.category_getter()?)
        }
    }

    fn category_getter(&self) -> Result<category::Getter> {
        category::Getter::new(&self.conn)
    }
}
