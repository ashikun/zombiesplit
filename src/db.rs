//! Top-level module for the model's sqlite database.

pub mod category;
pub mod error;
mod game;
mod init;
mod run;
use crate::model::{self, Time, attempt::Session, game::{category::ShortDescriptor, Config}, history, short::Name};
use std::{collections::HashMap, path::Path, sync::{RwLock, RwLockReadGuard, RwLockWriteGuard}};

pub use error::{Error, Result};
pub use run::Observer;
use rusqlite::Connection;

use self::category::{GcID, Locator};

/// A connection to zombiesplit's database.
pub struct Db {
    conn: RwLock<rusqlite::Connection>,
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
        Self {
            conn: RwLock::new(conn),
        }
    }

    fn lock_db_read(&self) -> Result<RwLockReadGuard<Connection>> {
        self.conn.read().map_err(|_| Error::Lock)
    }

    fn lock_db_write(&self) -> Result<RwLockWriteGuard<Connection>> {
        self.conn.write().map_err(|_| Error::Lock)
    }

    /// Initialises the database for first use.
    ///
    /// # Errors
    ///
    /// Propagates errors from the database if anything goes wrong.
    pub fn init(&self) -> Result<()> {
        init::on_db(self.lock_db_read()?)
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&self, short: &str, game: &Config) -> Result<()> {
        let mut conn = self.lock_db_write()?;
        let tx = conn.transaction()?;
        game::Inserter::new(&tx)?.add_game(short, game)?;
        Ok(tx.commit()?)
    }

    /// Adds the historic run `run` to the database.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a run
    /// fail.
    pub fn add_run<L: Locator>(&self, run: &history::TimedRun<L>) -> Result<()> {
        let run = run.with_locator(self.resolve_gcid(&run.category_locator)?);
        let mut conn = self.lock_db_write()?;
        let tx = conn.transaction()?;
        run::Inserter::new(&tx)?.add(&run)?;
        Ok(tx.commit()?)
    }

    /// Gets summaries for all game-category pairs in the database.
    ///
    /// # Errors
    ///
    /// Raises an error if the underlying SQL query fails.
    pub fn game_categories(&self) -> Result<Vec<model::game::category::Info>> {
        let conn = self.lock_db_read()?;
        let mut getter = category::Getter::new(&conn)?;
        getter.all_game_category_info()
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
        run::Finder::new(&*self.lock_db_read()?)?.runs_for(id)
    }

    pub fn split_pbs_for<L: Locator>(&self, loc: &L) -> Result<Vec<(Name, Time)>> {
        // TODO(@MattWindsor91): collate by segment
        let id = self.resolve_gcid(loc)?;
        let splits = self.split_id_map(id)?;

        Ok(
        run::Finder::new(&*self.lock_db_read()?)?.split_pbs_for(id)?.into_iter().map(
            |(id, x)| {
                (splits.get(&id).map(|x| x.to_owned()).unwrap_or_else(|| "??".to_owned()), x)
            }
        ).collect())
    }

    fn split_id_map(&self, gcid: GcID) -> Result<HashMap<i64, String>> {
        // TODO(@MattWindsor91): move this elsewhere?
        Ok(category::Getter::new(&*self.lock_db_read()?)?.splits(&gcid)?.into_iter().map(|s| (s.id, s.short)).collect())
    }

    /// Initialises a session for the game/category described by the given
    /// short descriptor.
    ///
    /// # Errors
    ///
    /// Fails if there is no such category for the given game in the database,
    /// or the game doesn't exist, or any other database error occurs.
    pub fn init_session(&self, desc: &ShortDescriptor) -> Result<Session> {
        let conn = self.lock_db_read()?;
        let mut getter = category::Getter::new(&conn)?;
        getter.init_session(&desc)
    }

    fn resolve_gcid<L: Locator>(&self, loc: &L) -> Result<GcID> {
        // TODO(@MattWindsor91): this is horrible.
        if let Some(x) = loc.as_game_category_id() {
            Ok(x)
        } else {
            let conn = self.lock_db_read()?;
            let mut getter = category::Getter::new(&conn)?;

            loc.locate(&mut getter)
        }
    }
}
