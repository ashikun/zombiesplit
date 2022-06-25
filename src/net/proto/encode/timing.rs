//! Timing encoding logic.
//!
//! This is the counterpart of the similarly named decoding module.
use super::super::{
    super::super::model::timing, dump_response::Total, Aggregate, Pace, RunDelta, SplitDelta,
};

/// Encodes a run total.
pub(super) fn total(total: timing::comparison::delta::Time) -> Total {
    let delta = Some(run_delta(&total.delta));
    let time = u32::from(total.time);
    Total { delta, time }
}

/// Encodes a split-level time delta.
pub(super) fn split_delta(d: &timing::comparison::delta::Split) -> SplitDelta {
    SplitDelta {
        pace: split_in_run_pace(d.pace) as i32,
        split_abs_delta: u32::from(d.split_abs_delta),
        run_abs_delta: u32::from(d.run_abs_delta),
    }
}

fn split_in_run_pace(pace: timing::comparison::pace::SplitInRun) -> Pace {
    use timing::comparison::pace::SplitInRun;
    match pace {
        SplitInRun::Inconclusive => Pace::None,
        SplitInRun::BehindAndLosing => Pace::Behind,
        SplitInRun::BehindAndGaining => Pace::BehindButGaining,
        SplitInRun::AheadAndLosing => Pace::AheadButLosing,
        SplitInRun::AheadAndGaining => Pace::Ahead,
        SplitInRun::SplitPersonalBest => Pace::PersonalBest,
    }
}

/// Encodes a run-level time delta.
pub(super) fn run_delta(d: &timing::comparison::Delta) -> RunDelta {
    let pace = pace(d.pace) as i32;
    let abs_delta = u32::from(d.abs_delta);
    RunDelta { pace, abs_delta }
}

fn pace(pace: timing::comparison::Pace) -> Pace {
    match pace {
        timing::comparison::Pace::Inconclusive => Pace::None,
        timing::comparison::Pace::Behind => Pace::Behind,
        timing::comparison::Pace::Ahead => Pace::Ahead,
        timing::comparison::Pace::PersonalBest => Pace::PersonalBest,
    }
}

/// Encodes a timestamp into a time.
pub(super) fn aggregate(agg: &timing::aggregate::Set) -> Aggregate {
    Aggregate {
        split: u32::from(agg.split),
        cumulative: u32::from(agg.cumulative),
    }
}
