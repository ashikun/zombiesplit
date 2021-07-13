//! High-level interface to zombiesplit.

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{db::category::{GcID, Locator}, model::{game::category::ShortDescriptor, history}};

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
        let run = history::run::FullyTimed::<ShortDescriptor>::from_toml_file(pbuf)?;
        self.db.add_run(&run)?;
        Ok(())
    }

    /// Lists all game-category pairs zombiesplit knows.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn list_game_categories(&self) -> Result<()> {
        for game in self.db.game_categories()? {
            println!("{} - {}: {}", game.short, game.game, game.category);
        }

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

    /// Lists all PBs for splits in the given game/category locator.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn split_pbs<L: Locator>(&self, loc: &L) -> Result<()> {
        // TODO(@MattWindsor91): decouple this from Zombie?

        for (short, time) in self.db.split_pbs_for(loc)? {
            println!("{}: {}", short, time);
        }

        Ok(())
    }

    /// Gets the run for the given game/category locator.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_pb<L: Locator>(&self, loc: &L, level: history::timing::Level) -> Result<()> {

        let handle = self.db.reader()?;
        let id = loc.locate(&mut handle.categories()?)?;

        let mut run = handle.runs()?;

        if let Some(pb) = run.run_pb_for(id)? {
            handle_run_pb(pb, &mut run, level)?;
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
        let mut session = self.session_from_db(short)?;
        session.add_observer(db::Observer::boxed(self.db.clone()));
        Ok(session)
    }

    fn session_from_db(&self, short: &ShortDescriptor) -> Result<model::attempt::Session> {
        let handle = self.db.reader()?;
        let mut cat = handle.categories()?;
        let session = cat.init_session(&short)?;
        Ok(session)
    }
}

fn handle_run_pb(pb: db::util::WithID<history::run::Summary<GcID>>, db: &mut db::run::Getter, level: history::timing::Level) -> Result<()> {
    // TODO(@MattWindsor91): decouple this from Zombie?
    println!("PB is {} on {}", pb.item.timing.total, pb.item.date);

    match level {
        history::timing::Level::Summary => (),
        history::timing::Level::Totals => handle_run_totals(pb, db)?,
        history::timing::Level::Full => println!("full timing not yet implemented"),
    }

    Ok(())
}

fn handle_run_totals(run: db::util::WithID<history::run::Summary<GcID>>, db: &mut db::run::Getter) -> Result<()> {
    let run = db.add_split_totals(run)?;

    println!("Split totals:");

    // TODO(@MattWindsor91): order these
    for (short, time) in run.item.timing.totals {
        println!("{}: {}", short, time);
    }

    Ok(())
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
