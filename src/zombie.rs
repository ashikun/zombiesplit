/*! High-level interface to the zombiesplit database.

The contents of this file will likely eventually be broken up into smaller portions. */

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use tabwriter::TabWriter;
use thiserror::Error;

use super::{
    db::{self, category::Locator, Db},
    model::{
        self,
        game::category::ShortDescriptor,
        history::{self, run::ForLevel, timing::Timing},
        load::Loadable,
        short,
        timing::comparison::{self, provider::Provider, Comparison},
    },
};

/// Adds the game with the given path to the zombiesplit database.
///
/// # Errors
///
/// Returns any errors from the database, or if the given path is
/// ill-formed.
pub fn add_game<P: AsRef<Path>>(db: &mut Db, path: P) -> Result<()> {
    let pbuf = ensure_toml(path);
    let short = deduce_short(&pbuf)?;
    let game = model::game::Config::from_toml_file(pbuf)?;
    db.add_game(&short, &game)?;
    Ok(())
}

/// Adds the run with the given path to the zombiesplit database.
///
/// # Errors
///
/// Returns any errors from the database, or if the given path is
/// ill-formed.
pub fn add_run<P: AsRef<Path>>(db: &mut Db, path: P) -> Result<()> {
    let pbuf = ensure_toml(path);
    let run = history::run::FullyTimed::<ShortDescriptor>::from_toml_file(pbuf)?;
    db.add_run(&run)?;
    Ok(())
}

/// Lists all game-category pairs zombiesplit knows.
///
/// # Errors
///
/// Returns any database errors occurring during the listing.
pub fn list_game_categories(db: &Db) -> Result<()> {
    for game in db.game_categories()? {
        println!("{} - {}: {}", game.short, game.game, game.category);
    }

    Ok(())
}

/// Lists all runs for the given game/category locator.
///
/// # Errors
///
/// Returns any database errors occurring during the listing.
pub fn list_runs(db: &Db, loc: &impl Locator) -> Result<()> {
    // TODO(@MattWindsor91): decouple this from Zombie?
    use colored::Colorize;

    for run in db.runs_for(loc)? {
        let rank = match run.timing.rank {
            None => "n/a".red(),
            Some(1) => "1".yellow(),
            Some(k) => format!("{k}").green(),
        };

        println!("{rank}. {} on {}", run.timing.total, run.date);
    }

    Ok(())
}

/// Lists all PBs for splits in the given game/category locator.
///
/// # Errors
///
/// Returns any database errors occurring during the listing.
pub fn split_pbs(db: &Db, loc: &impl Locator) -> Result<()> {
    let handle = db.reader()?;
    let mut insp = handle.inspect(loc)?;
    if let Some(c) = insp.comparison()? {
        emit_split_pbs(c)?;
    }

    Ok(())
}

fn emit_split_pbs(c: Comparison) -> Result<()> {
    // TODO(@MattWindsor91): decouple this from Zombie?
    let mut tw = TabWriter::new(std::io::stdout());
    writeln!(tw, "SPLIT\tSPLIT PB\tIN-RUN PB")?;
    for (short, split) in c {
        output_split_pb(&mut tw, short, split)?;
    }
    tw.flush()?;

    Ok(())
}

/// Gets the run at the given index for the given game/category locator.
///
/// # Errors
///
/// Returns any database errors occurring during the listing.
pub fn run_at_index(
    db: &Db,
    loc: &impl Locator,
    index: usize,
    level: history::timing::Level,
) -> Result<()> {
    let handle = db.reader()?;
    let mut insp = handle.inspect(loc)?;
    if let Some(run) = insp.run_at_index(index, &level)? {
        // TODO(@MattWindsor91): decouple
        output_run(run);
    }

    Ok(())
}

/// Gets the PB run for the given game/category locator.
///
/// # Errors
///
/// Returns any database errors occurring during the listing.
pub fn run_pb(db: &Db, loc: &impl Locator, level: history::timing::Level) -> Result<()> {
    let handle = db.reader()?;
    let mut insp = handle.inspect(loc)?;
    if let Some(pb) = insp.run_pb(&level)? {
        // TODO(@MattWindsor91): decouple
        output_run(pb);
    }

    Ok(())
}

fn output_run(run: ForLevel<super::db::category::GcID>) {
    let status = if run.was_completed {
        "complete"
    } else {
        "incomplete"
    };

    println!("{} on {} ({})", run.timing.total(), run.date, status);

    if let history::timing::ForLevel::Totals(totals) = run.timing {
        // TODO(@MattWindsor91): order by position
        for (split, total) in totals.totals {
            println!("{split}: {total}");
        }
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
    #[error("error getting comparison info")]
    Comparison(#[from] model::timing::comparison::provider::Error),
    #[error("error loading data from file")]
    GameLoad(#[from] model::load::Error),
    #[error("couldn't deduce game short-name")]
    MissingShort,
}

/// The top-level zombiesplit result type.
pub type Result<T> = std::result::Result<T, Error>;

fn output_split_pb(
    tw: &mut impl Write,
    short: short::Name,
    split: comparison::Split,
) -> Result<()> {
    write!(tw, "{short}\t")?;
    write!(tw, "{0}", split.split_pb)?;
    write!(tw, "\t")?;
    write!(tw, "{0}", split.in_pb_run.split)?;
    writeln!(tw)?;

    Ok(())
}
