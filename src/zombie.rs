//! High-level interface to zombiesplit.

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    db::category::Locator,
    model::{attempt::Observer, game::category::ShortDescriptor, history::FullTiming},
};

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
    db: Rc<db::Db>,
}

impl Zombie {
    /// Constructs a new instance of zombiesplit, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::System) -> Result<Self> {
        let db = Rc::new(db::Db::new(&cfg.db_path)?);
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
        let run = model::history::Run::<ShortDescriptor, FullTiming>::from_toml_file(pbuf)?;
        self.db.add_run(&run)?;
        Ok(())
    }

    /// Lists all runs for the given game/category locator.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn list_runs<L: Locator>(&self, loc: &L) -> Result<()> {
        // TODO(@MattWindsor91): decouple this from Zombie?
        use colored::Colorize;

        for run in self.db.runs_for(loc)? {
            let rank = match run.timing.rank {
                None => "n/a".red(),
                Some(1) => "1".yellow(),
                Some(k) => format!("{}", k).green(),
            };

            println!("{}. {} on {}", rank, run.timing.total, run.date)
        }

        Ok(())
    }

    /// Opens a split UI session for the given game/category descriptor.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn run(self, desc: &ShortDescriptor) -> Result<()> {
        let session = self.session(desc)?;
        let p = Presenter::new(session);
        view::View::new(self.cfg.ui)?.spawn(p)?.run()?;
        Ok(())
    }

    fn session(&self, short: &ShortDescriptor) -> Result<model::attempt::Session> {
        let mut session = self.db.init_session(&short)?;
        session.add_observer(self.db_observer());
        Ok(session)
    }

    fn db_observer(&self) -> Box<dyn Observer> {
        Box::new(crate::db::Observer::new(self.db.clone()))
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
