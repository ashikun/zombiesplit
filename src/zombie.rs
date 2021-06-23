//! High-level interface to zombiesplit.

use std::path::{Path, PathBuf};

use crate::model::game::category::ShortDescriptor;

use super::{
    config, db,
    model::{self, load::Loadable},
    presenter::Presenter,
    view,
};
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
        let pbuf = ensure_toml(path);
        let short = deduce_short(&pbuf)?;
        let game = model::game::Config::from_toml_file(pbuf)?;
        self.db.add_game(&short, &game)?;
        Ok(())
    }

    /// Adds the run with the given path to the zombiesplit database.
    ///
    /// # Errors
    ///
    /// Returns any errors from the database, or if the given path is
    /// ill-formed.
    pub fn add_run<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let pbuf = ensure_toml(path);
        let run = model::history::Run::<ShortDescriptor>::from_toml_file(pbuf)?;
        self.db.add_run(&run)?;
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

fn ensure_toml<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if path.extension().is_none() {
        path.with_extension("toml")
    } else {
        path.to_owned()
    }
}

fn deduce_short(path: &Path) -> Result<String> {
    Ok(path
        .file_stem()
        .ok_or(Error::MissingShort)?
        .to_string_lossy()
        .into_owned())
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
