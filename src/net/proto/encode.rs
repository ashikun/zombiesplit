//! Helpers for encoding zombiesplit models to protobuf equivalents.

use super::super::{
    super::model::{
        attempt::{observer, split, Run, Split},
        game::category,
        timing,
    },
    dump,
};
use crate::model::timing::comparison::Pace;
use itertools::Itertools;

/// Shorthand for encoding errors.
///
/// These invariably result in gRPC status codes on failure.
pub type Result<T> = std::result::Result<T, tonic::Status>;

/// Encodes a dump into a protobuf dump response.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn dump(dump: &dump::Dump) -> Result<super::DumpResponse> {
    Ok(super::DumpResponse {
        server: Some(dump_server(&dump.server)),
        run: Some(dump_run(&dump.run)?),
    })
}

fn dump_server(server: &dump::Server) -> super::dump_response::Server {
    super::dump_response::Server {
        ident: server.ident.clone(),
        version: Some(dump_version(&server.version)),
    }
}

fn dump_version(version: &semver::Version) -> super::dump_response::server::Version {
    super::dump_response::server::Version {
        major: version.major,
        minor: version.minor,
        patch: version.patch,
    }
}

fn dump_run(run: &Run) -> Result<super::dump_response::Run> {
    Ok(super::dump_response::Run {
        game_category: Some(game_category(&run.metadata)),
        attempt_info: Some(attempt_info(&run.attempt)?),
        splits: splits(&run.splits),
    })
}

fn game_category(info: &category::Info) -> super::dump_response::run::GameCategory {
    super::dump_response::run::GameCategory {
        category_name: info.category.clone(),
        game_name: info.game.clone(),
        category_sid: info.short.category.to_string(),
        game_sid: info.short.game.to_string(),
    }
}

fn splits(splits: &split::Set) -> Vec<super::dump_response::run::Split> {
    splits.iter().map(split).collect_vec()
}

fn split(split: &split::Split) -> super::dump_response::run::Split {
    super::dump_response::run::Split {
        sid: split.info.short.to_string(),
        // id is intentionally lost; the client doesn't need to know about it.
        name: split.info.name.clone(),
        times: milli_times(split),
    }
}

fn milli_times(split: &Split) -> Vec<u32> {
    split.all_times().into_iter().map(u32::from).collect_vec()
}

/// Encodes an observer-level event into a protobuf event.
///
/// # Errors
///
/// Fails with `out_of_range` if any attempt counts cannot be stored as 64-bit integers.
pub fn event(event: &observer::Event) -> Result<super::Event> {
    Ok(super::Event {
        payload: match event {
            observer::Event::Total(t) => Some(super::event::Payload::Total(total(t))),
            observer::Event::NumSplits(_) => None,
            observer::Event::Reset(info) => Some(super::event::Payload::Reset(attempt_info(info)?)),
            observer::Event::GameCategory(_) => None,
            observer::Event::Split(_, _) => None,
        },
    })
}

/// Encodes a total into its protobuf form.
///
/// Split aggregates and run totals share a protobuf representation.
fn total(total: &observer::Total) -> super::Aggregate {
    let (time, ty) = match total {
        observer::Total::Attempt(t) => (t.time, super::aggregate::Type::Attempt),
        observer::Total::Comparison(t) => (*t, super::aggregate::Type::Comparison),
        observer::Total::SumOfBest(t) => (*t, super::aggregate::Type::SumOfBest),
    };
    let mut agg = super::Aggregate {
        r#type: ty as i32,
        value: u32::from(time),
        pace: 0,
    };
    if let observer::Total::Attempt(t) = total {
        if let Some(p) = pace(t.pace) {
            agg.set_pace(p);
        }
    }
    agg
}

fn pace(pace: timing::comparison::Pace) -> Option<super::Pace> {
    match pace {
        Pace::Inconclusive => None,
        Pace::Behind => Some(super::Pace::Behind),
        Pace::Ahead => Some(super::Pace::Ahead),
        Pace::PersonalBest => Some(super::Pace::Ahead),
    }
}

/// Encodes attempt information into its protobuf form.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn attempt_info(attempt: &category::AttemptInfo) -> Result<super::AttemptInfo> {
    Ok(super::AttemptInfo {
        total: try_from_range(attempt.total)?,
        completed: try_from_range(attempt.completed)?,
    })
}

fn try_from_range<E: ToString, X, Y: TryFrom<X, Error = E>>(
    x: X,
) -> std::result::Result<Y, tonic::Status> {
    Y::try_from(x).map_err(|e| tonic::Status::out_of_range(e.to_string()))
}
