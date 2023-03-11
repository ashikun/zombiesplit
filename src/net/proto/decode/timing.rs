//! Timing decoding logic.
//!
//! This is the counterpart of the similarly named encoding module.
use super::{
    super::{
        super::super::model::timing, dump_response::Total, Aggregate, Pace, RunDelta, SplitDelta,
    },
    error::Result,
};

/// Decodes a run-wide total.
///
/// # Errors
///
/// Fails with `out_of_range` if any timestamp is too large to represent a valid times or deltas,
/// and `invalid_argument` if there is any other error in decoding paces or times.
pub(super) fn total(t: &Total) -> Result<timing::comparison::delta::Time> {
    let delta = t
        .delta
        .as_ref()
        .map(run_delta)
        .transpose()?
        .unwrap_or_default();
    let time = time(t.time);
    timing::comparison::delta::Time {
        delta,
        time: t.time,
    }
}

/// Decodes a split-level time delta.
///
/// # Errors
///
/// Fails with `out_of_range` if the timestamp is too large to represent a valid delta amount,
/// and `invalid_argument` if there is any other error in decoding the amount or the pace.
pub(super) fn split_delta(d: &SplitDelta) -> Result<timing::comparison::delta::Split> {
    Ok(timing::comparison::delta::Split {
        pace: split_in_run_pace(d.pace()),
        split_abs_delta: timing::time::Time::from_millis(d.split_abs_delta)?,
        run_abs_delta: timing::time::Time::from_millis(d.run_abs_delta)?,
    })
}

fn split_in_run_pace(pace: Pace) -> timing::comparison::pace::SplitInRun {
    match pace {
        Pace::None => timing::comparison::pace::SplitInRun::Inconclusive,
        Pace::Behind => timing::comparison::pace::SplitInRun::BehindAndLosing,
        Pace::BehindButGaining => timing::comparison::pace::SplitInRun::BehindAndGaining,
        Pace::AheadButLosing => timing::comparison::pace::SplitInRun::AheadAndLosing,
        Pace::Ahead => timing::comparison::pace::SplitInRun::AheadAndGaining,
        Pace::PersonalBest => timing::comparison::pace::SplitInRun::SplitPersonalBest,
    }
}

/// Decodes a run-level time delta.
///
/// # Errors
///
/// Fails with `out_of_range` if the timestamp is too large to represent a valid delta amount,
/// and `invalid_argument` if there is any other error in decoding the amount or the pace.
pub(super) fn run_delta(d: &RunDelta) -> Result<timing::comparison::Delta> {
    let pace = pace(d.pace());
    let abs_delta = timing::time::Time::from_millis(d.abs_delta)?;
    Ok(timing::comparison::Delta { pace, abs_delta })
}

fn pace(pace: Pace) -> timing::comparison::Pace {
    match pace {
        Pace::None => timing::comparison::Pace::Inconclusive,
        Pace::Behind | Pace::BehindButGaining => timing::comparison::Pace::Behind,
        Pace::Ahead | Pace::AheadButLosing => timing::comparison::Pace::Ahead,
        Pace::PersonalBest => timing::comparison::Pace::PersonalBest,
    }
}

/// Decodes an aggregate set.
pub(super) fn aggregate(agg: &Aggregate) -> timing::aggregate::Set {
    timing::aggregate::Set {
        split: agg.split.as_ref().map(time).unwrap_or_default(),
        cumulative: agg.cumulative.as_ref().map(time).unwrap_or_default(),
    }
}

/// Decodes a time.
pub(super) fn time(t: &super::super::Time) -> timing::Time {
    timing::Time::from_millis(t.millis)
}
