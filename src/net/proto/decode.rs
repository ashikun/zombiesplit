//! Helpers for decoding protobuf representations of zombiesplit models.

pub mod attempt;
pub mod comparison;
pub mod error;
pub mod event;

use crate::model::{
    attempt::{action, session},
    game::category,
    timing,
};
pub use error::{Error, Missing, Result, Unknown};

/// Decodes a protobuf representation of a dump.
///
/// # Errors
///
/// Fails if the attempt counts cannot be stored as `usize`, or if anything is missing from the
/// dump that we expect to see.
pub fn dump(dump: &super::DumpResponse) -> Result<session::State> {
    Ok(session::State {
        run: attempt::decode(Missing::Attempt.require(dump.attempt.as_ref())?)?,
        comparison: dump
            .comparison
            .as_ref()
            .map(comparison::decode)
            .transpose()?
            .unwrap_or_default(),
    })
}

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

fn pace(pace: super::Pace) -> timing::comparison::Pace {
    match pace {
        super::Pace::None => timing::comparison::Pace::Inconclusive,
        super::Pace::Behind | super::Pace::BehindButGaining => timing::comparison::Pace::Behind,
        super::Pace::Ahead | super::Pace::AheadButLosing => timing::comparison::Pace::Ahead,
        super::Pace::PersonalBest => timing::comparison::Pace::PersonalBest,
    }
}

/// Decodes a push action.
///
/// # Errors
///
/// Fails if the split index or time are out of bounds.
pub fn push_action(request: &super::PushRequest) -> Result<action::Action> {
    Ok(action::Action::Push(
        split_index(request.index)?,
        time(request.time)?,
    ))
}

/// Decodes a pop action.
///
/// # Errors
///
/// Fails if the split index is out of bounds, or the pop type is malformed.
pub fn pop_action(request: &super::PopRequest) -> Result<action::Action> {
    Ok(action::Action::Pop(
        split_index(request.index)?,
        pop(request.r#type)?,
    ))
}

/// Tries to interpret `pop_index` as a protobuf pop type, and decode it into the model form.
///
/// # Errors
///
/// Fails with `Missing` if `pop_index` doesn't correspond to a valid pop type.
fn pop(pop_index: i32) -> Result<action::Pop> {
    Ok(
        match Unknown::Pop.require(super::Pop::from_i32(pop_index))? {
            super::Pop::One => action::Pop::One,
            super::Pop::All => action::Pop::All,
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

/// Decodes a timestamp into a time.
///
/// # Errors
///
/// Fails with `out_of_range` if the timestamp is too large to represent a valid time, and
/// `invalid_argument` if there is any other error in decoding the time.
pub fn time(stamp: u32) -> Result<timing::Time> {
    Ok(timing::Time::try_from(stamp)?)
}
