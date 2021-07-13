//! Top-level module for the model's sqlite database.

pub mod category;
pub mod error;
mod game;
mod init;
pub mod run;
pub mod util;
use crate::model::{self, game::Config, history, short::Name, Time};
use std::{
    collections::HashMap,
    path::Path,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

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

    /// Gets a read handle to this database.
    ///
    /// # Errors
    ///
    /// Returns an error if we can't acquire a handle for the database for
    /// some reason.
    pub fn reader(&self) -> Result<Reader> {
        // TODO(@MattWindsor91): refactor all db reads to go through this
        Ok(Reader {
            conn: self.lock_db_read()?,
        })
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

    /// Performs some activity using a run getter.
    ///
    /// # Errors
    ///
    /// Raises errors on trying to acquire the getter, or when calling `f`.
    pub fn get_from_runs<T>(&self, f: impl FnOnce(&run::Getter) -> Result<T>) -> Result<T> {
        let conn = self.lock_db_read()?;
        let getter = run::Getter::new(&conn)?;
        f(&getter)
    }

    /// Performs some activity using a category getter.
    ///
    /// # Errors
    ///
    /// Raises errors on trying to acquire the getter, or when calling `f`.
    pub fn get_from_categories<T>(
        &self,
        f: impl FnOnce(&category::Getter) -> Result<T>,
    ) -> Result<T> {
        let conn = self.lock_db_read()?;
        let getter = category::Getter::new(&conn)?;
        f(&getter)
    }

    /// Adds the historic run `run` to the database.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a run
    /// fail.
    pub fn add_run<L: Locator>(&self, run: &history::run::FullyTimed<L>) -> Result<()> {
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
    /// Raises an error if any of the SQL queries relating to getting a run
    /// fail.
    pub fn runs_for<L: Locator>(&self, loc: &L) -> Result<Vec<history::run::Summary<GcID>>> {
        let id = self.resolve_gcid(loc)?;
        let runs = run::Getter::new(&*self.lock_db_read()?)?.runs_for(id)?;
        Ok(runs.into_iter().map(|x| x.item).collect())
    }

    /// Gets the PB run for the game-category located by `loc`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to getting a run
    /// fail.
    pub fn run_pb_for<L: Locator>(&self, loc: &L) -> Result<Option<history::run::Summary<GcID>>> {
        let id = self.resolve_gcid(loc)?;
        Ok(run::Getter::new(&*self.lock_db_read()?)?
            .run_pb_for(id)?
            .map(|x| x.item))
    }

    /// Gets split PBs for the game-category located by `loc`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to getting a run
    /// fail.
    pub fn split_pbs_for<L: Locator>(&self, loc: &L) -> Result<Vec<(Name, Time)>> {
        // TODO(@MattWindsor91): collate by segment
        let id = self.resolve_gcid(loc)?;
        let splits = self.split_id_map(id)?;

        Ok(run::Getter::new(&*self.lock_db_read()?)?
            .split_pbs_for(id)?
            .into_iter()
            .map(|s| {
                (
                    splits
                        .get(&s.id)
                        .map_or_else(|| "??".to_owned(), String::clone),
                    s.item,
                )
            })
            .collect())
    }

    fn split_id_map(&self, gcid: GcID) -> Result<HashMap<i64, String>> {
        // TODO(@MattWindsor91): move this elsewhere?
        Ok(category::Getter::new(&*self.lock_db_read()?)?
            .splits(&gcid)?
            .into_iter()
            .map(|s| (s.id, s.short))
            .collect())
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

/// A handle used to perform read operations on the database.
pub struct Reader<'conn> {
    conn: RwLockReadGuard<'conn, rusqlite::Connection>,
}

impl<'conn> Reader<'conn> {
    /// Gets an interface to the category database.
    ///
    /// # Errors
    ///
    /// Errors if we can't construct the database queries.
    pub fn categories(&self) -> Result<category::Getter> {
        category::Getter::new(&*self.conn)
    }

    /// Gets an interface to the historic runs database.
    ///
    /// # Errors
    ///
    /// Errors if we can't construct the database queries.
    pub fn runs(&self) -> Result<run::Getter> {
        run::Getter::new(&*self.conn)
    }
}
