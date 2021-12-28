//! Utility functions for the zombiesplit command line interfaces.

use crate::model::game::category::ShortDescriptor;
use anyhow;
use std::process::exit;
use thiserror::Error;

pub fn handle_error(res: anyhow::Result<()>) {
    if let Err(e) = res {
        eprintln!("error: {:?}", e);
        exit(1)
    }
}

/// Gets the short descriptor from a clap match set.
///
/// # Errors
///
/// Fails if the match set does not have a 'game' key or a 'category' key.
pub fn get_short_descriptor(matches: &clap::ArgMatches) -> Result<ShortDescriptor> {
    let game = matches.value_of("game").ok_or(Error::Game)?;
    let category = matches.value_of("category").ok_or(Error::Category)?;
    Ok(ShortDescriptor::new(game, category))
}

/// Errors returned by the CLI.
#[derive(Debug, Error)]
pub enum Error {
    /// Error getting a category from the command line.
    #[error("no category provided")]
    Category,
    /// Error getting a game from the command line.
    #[error("no game provided")]
    Game,
    /// Error getting a run from the command line.
    #[error("no run provided")]
    Run,
}

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;
