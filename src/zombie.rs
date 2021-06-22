//! High-level interface to zombiesplit.

use std::path::Path;

use crate::model::game::category::ShortDescriptor;

use super::{config, db, model::{self, load::Loadable}, presenter::Presenter, view};
use thiserror::Error;

/// High-level interface to zombiesplit.
///
/// This struct wraps most of zombiesplit's functionality in a way that is
/// easy for the command-line app to handle.
pub struct Zombie {
    cfg: config::System,
    db: db::Db,
}

impl Zombie {
    /// Constructs a new instance of zombiesplit, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::System) -> Result<Self> {
        let db = db::Db::new(&cfg.db_path)?;
        Ok(Zombie { cfg, db })
    }

    /// Initialises the zombiesplit database.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to initialise the database.
    pub fn init_db(&self) -> Result<()> {
        Ok(self.db.init()?)
    }

    /// Adds the game with the given path to the zombiesplit database.
    ///
    /// # Errors
    ///
    /// Returns any errors from the database, or if the given path is
    /// ill-formed.
    pub fn add_game<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let pbuf = path.as_ref().with_extension("toml");
        let short = pbuf
            .file_stem()
            .ok_or(Error::MissingShort)?
            .to_string_lossy()
            .into_owned();
        let game = model::game::Config::from_toml_file(pbuf)?;
        self.db.add_game(&short, &game)?;
        Ok(())
    }

    /// Opens a split UI session for the given game/category descriptor.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn run(self, desc: &ShortDescriptor) -> Result<()> {
        let session = self.db.init_session(&desc)?;
        let p = Presenter::new(session);
        view::View::new(self.cfg.ui)?.spawn(p)?.run()?;
        Ok(())
    }
}

/// The top-level zombiesplit error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("database error")]
    Db(#[from] db::Error),
    #[error("UI error")]
    View(#[from] view::Error),
    #[error("error loading data from file")]
    GameLoad(#[from] model::load::Error),
    #[error("couldn't deduce game short-name")]
    MissingShort,
}

/// The top-level zombiesplit result type.
pub type Result<T> = std::result::Result<T, Error>;
