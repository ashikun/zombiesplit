//! High-level interface to zombiesplit.

use std::{
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    db::inspect::Inspector,
    model::{
        comparison::{self, Provider},
        Time,
    },
};

use super::{
    config,
    db::{self, category::Locator},
    model::{
        self,
        game::category::ShortDescriptor,
        history::{self, timing::Timing},
        load::Loadable,
    },
    ui,
};
use tabwriter::TabWriter;
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
    pub fn split_pbs(&self, loc: &impl Locator) -> Result<()> {
        // TODO(@MattWindsor91): decouple this from Zombie?
        let handle = self.db.reader()?;
        let mut insp = handle.inspect(loc)?;

        // TODO(@MattWindsor91): decouple this from Zombie?
        let mut tw = TabWriter::new(std::io::stdout());
        writeln!(tw, "SPLIT\tSPLIT PB\tIN-RUN PB")?;
        for (short, split) in insp.comparison().into_iter().flat_map(|x| x.splits) {
            output_split_pb(&mut tw, &short, split)?;
        }
        tw.flush()?;

        Ok(())
    }

    /// Gets the run for the given game/category locator.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_pb<L: Locator>(&self, loc: &L, level: history::timing::Level) -> Result<()> {
        let handle = self.db.reader()?;
        let mut insp = handle.inspect(loc)?;
        if let Some(pb) = insp.run_pb(level)? {
            // TODO(@MattWindsor91): decouple
            println!("PB is {} on {}", pb.timing.total(), pb.date);

            if let history::timing::ForLevel::Totals(totals) = pb.timing {
                // TODO(@MattWindsor91): order by position
                for (split, total) in totals.totals {
                    println!("{}: {}", split, total)
                }
            }
        }

        Ok(())
    }

    /// Opens a split UI session for the given game/category descriptor.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn run(self, desc: &ShortDescriptor) -> Result<()> {
        let handle = self.db.reader()?;
        let insp = handle.inspect(desc)?;

        let session = self.session(insp)?;
        ui::run(self.cfg.ui, session)?;
        Ok(())
    }

    fn session<'a>(&self, mut insp: Inspector<'a>) -> Result<model::attempt::Session<'a>> {
        let mut session = insp.init_session()?;
        session.observers.add(db::Observer::boxed(self.db.clone()));
        session.set_comparison_provider(Box::new(insp));
        Ok(session)
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
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("UI error")]
    View(#[from] ui::view::Error),
    #[error("error loading data from file")]
    GameLoad(#[from] model::load::Error),
    #[error("couldn't deduce game short-name")]
    MissingShort,
}

/// The top-level zombiesplit result type.
pub type Result<T> = std::result::Result<T, Error>;

fn output_split_pb(tw: &mut impl Write, short: &str, split: comparison::Split) -> Result<()> {
    write!(tw, "{}\t", short)?;
    output_time(tw, split.split)?;
    write!(tw, "\t")?;
    output_time(tw, split.in_run.map(|x| x.time))?;
    writeln!(tw)?;

    Ok(())
}

fn output_time(tw: &mut impl Write, time: Option<Time>) -> Result<()> {
    if let Some(t) = time {
        write!(tw, "{}", t)?;
    } else {
        write!(tw, "--")?;
    }
    Ok(())
}
