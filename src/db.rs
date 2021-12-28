//! Top-level module for the model's sqlite database.

pub mod category;
pub mod error;
mod game;
mod init;
pub mod inspect;
pub mod run;
pub mod util;
use crate::model::{self, game::Config, history};
use std::path::Path;

pub use error::{Error, Result};
use r2d2::ManageConnection;
use r2d2_sqlite::SqliteConnectionManager;
pub use run::Observer;

use self::{
    category::{GcID, Locator},
    inspect::Inspector,
};

/// A root connection to zombiesplit's database.
pub struct Db {
    manager: r2d2_sqlite::SqliteConnectionManager,
}

impl Db {
    /// Opens a database connection to a given file.
    ///
    /// # Errors
    ///
    /// Returns errors from the underlying database library if the connection
    /// opening failed.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            manager: SqliteConnectionManager::file(path), // TODO(@MattWindsor91): r2d2 connection pool
        })
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
            conn: self.manager.connect()?,
        })
    }

    /// Initialises the database for first use.
    ///
    /// # Errors
    ///
    /// Propagates errors from the database if anything goes wrong.
    pub fn init(&self) -> Result<()> {
        init::on_db(&self.manager.connect()?)
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&self, short: &str, game: &Config) -> Result<()> {
        let mut conn = self.manager.connect()?;
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
    pub fn add_run<L: Locator>(&self, run: &history::run::FullyTimed<L>) -> Result<()> {
        let run = run.with_locator(self.resolve_gcid(&run.category_locator)?);
        let mut conn = self.manager.connect()?;
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
        let conn = self.manager.connect()?;
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
        let runs = run::Getter::new(&self.manager.connect()?)?.runs_for(id)?;
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
        Ok(run::Getter::new(&self.manager.connect()?)?
            .run_pb_for(id)?
            .map(|x| x.item))
    }

    fn resolve_gcid<L: Locator>(&self, loc: &L) -> Result<GcID> {
        // TODO(@MattWindsor91): this is horrible.
        if let Some(x) = loc.as_game_category_id() {
            Ok(x)
        } else {
            let conn = self.manager.connect()?;
            let mut getter = category::Getter::new(&conn)?;

            loc.locate_gcid(&mut getter)
        }
    }
}

/// A handle used to perform read operations on the database.
pub struct Reader {
    conn: rusqlite::Connection,
}

impl Reader {
    /// Gets an interface to the category database.
    ///
    /// # Errors
    ///
    /// Errors if we can't construct the database queries.
    pub fn categories(&self) -> Result<category::Getter> {
        category::Getter::new(&self.conn)
    }

    /// Gets an interface to the historic runs database.
    ///
    /// # Errors
    ///
    /// Errors if we can't construct the database queries.
    pub fn runs(&self) -> Result<run::Getter> {
        run::Getter::new(&self.conn)
    }

    /// Gets an inspector over the database.
    ///
    /// # Errors
    ///
    /// Errors if we can't construct the low-level getters.
    pub fn inspect(&self, loc: &impl Locator) -> Result<Inspector> {
        let mut cat = self.categories()?;
        Ok(Inspector {
            info: loc.locate(&mut cat)?,
            run: self.runs()?,
            cat,
        })
    }
}
