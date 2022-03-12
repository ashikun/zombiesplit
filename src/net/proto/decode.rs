//! Helpers for decoding protobuf representations of zombiesplit models.

pub mod action;
pub mod attempt;
pub mod comparison;
pub mod dump;
pub mod error;
pub mod event;

use super::super::super::model::{game::category, session, timing};
pub use error::{Error, Missing, Result, Unknown};

/// Decodes a protobuf representation of attempt information into its model form.
///
/// # Errors
///
/// Fails if the attempt counts cannot be stored as `usize`.
pub fn attempt_info(attempt: &super::AttemptInfo) -> Result<category::AttemptInfo> {
    Ok(category::AttemptInfo {
        total: usize::try_from(attempt.total)?,
        completed: usize::try_from(attempt.completed)?,
    })
}

fn split_in_run_pace_from_index(index: i32) -> Result<timing::comparison::pace::SplitInRun> {
    Unknown::Pace
        .require(super::Pace::from_i32(index))
        .map(split_in_run_pace)
}

fn split_in_run_pace(pace: super::Pace) -> timing::comparison::pace::SplitInRun {
    match pace {
        super::Pace::None => timing::comparison::pace::SplitInRun::Inconclusive,
        super::Pace::Behind => timing::comparison::pace::SplitInRun::BehindAndLosing,
        super::Pace::BehindButGaining => timing::comparison::pace::SplitInRun::BehindAndGaining,
        super::Pace::AheadButLosing => timing::comparison::pace::SplitInRun::AheadAndLosing,
        super::Pace::Ahead => timing::comparison::pace::SplitInRun::AheadAndGaining,
        super::Pace::PersonalBest => timing::comparison::pace::SplitInRun::SplitPersonalBest,
    }
}

fn pace_from_index(index: i32) -> Result<timing::comparison::Pace> {
    Unknown::Pace
        .require(super::Pace::from_i32(index))
        .map(pace)
}

fn pace(pace: super::Pace) -> timing::comparison::Pace {
    match pace {
        super::Pace::None => timing::comparison::Pace::Inconclusive,
        super::Pace::Behind | super::Pace::BehindButGaining => timing::comparison::Pace::Behind,
        super::Pace::Ahead | super::Pace::AheadButLosing => timing::comparison::Pace::Ahead,
        super::Pace::PersonalBest => timing::comparison::Pace::PersonalBest,
    }
}

/// Tries to interpret `pop_index` as a protobuf pop type, and decode it into the model form.
///
/// # Errors
///
/// Fails with `Missing` if `pop_index` doesn't correspond to a valid pop type.
fn pop(pop_index: i32) -> Result<session::action::Pop> {
    Ok(
        match Unknown::Pop.require(super::Pop::from_i32(pop_index))? {
            super::Pop::One => session::action::Pop::One,
            super::Pop::All => session::action::Pop::All,
        },
    )
}

/// Decodes a protobuf representation of a split index into a model representation.
///
/// This is a straightforward integer bit width change.
///
/// # Errors
///
/// Fails with `out_of_range` if the index is too large (which may happen on eg. 32-bit systems).
pub fn split_index(index: u64) -> Result<usize> {
    Ok(usize::try_from(index)?)
}

/// Decodes an aggregate set.
///
/// # Errors
///
/// Fails with `out_of_range` if any of the timestamps are too large to represent a valid time, and
/// `invalid_argument` if there are any other errors in decoding the time.
pub fn aggregate(agg: &super::Aggregate) -> Result<timing::aggregate::Set> {
    Ok(timing::aggregate::Set {
        split: time(agg.split)?,
        cumulative: time(agg.cumulative)?,
    })
}

/// Decodes a timestamp into a time.
///
/// # Errors
///
/// Fails with `out_of_range` if the timestamp is too large to represent a valid time, and
/// `invalid_argument` if there is any other error in decoding the time.
pub fn time(stamp: u32) -> Result<timing::Time> {
    Ok(timing::Time::try_from(stamp)?)
}
