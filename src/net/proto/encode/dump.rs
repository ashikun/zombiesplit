//! Top-level encoding for state dumps.

use super::{
    super::{
        super::super::model::{session, short},
        dump_response, DumpResponse,
    },
    attempt, comparison, Result,
};
use crate::model::session::State;
use std::collections::HashMap;

/// Encodes a dump into a protobuf response.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn encode(dump: &session::State) -> Result<DumpResponse> {
    Ok(DumpResponse {
        attempt: Some(attempt::encode(&dump.run)?),
        comparison: Some(comparison::encode(&dump.comparison)),
        notes: notes(&dump.notes),
        pace: pace(dump),
        total: total(dump),
    })
}

fn notes(
    notes: &short::Map<session::state::SplitNote>,
) -> HashMap<String, dump_response::SplitNote> {
    notes
        .iter()
        .map(|(short, n)| (short.to_string(), note(n)))
        .collect()
}

fn note(note: &session::state::SplitNote) -> dump_response::SplitNote {
    dump_response::SplitNote {
        aggregate: Some(super::aggregate(&note.aggregates)),
        pace: super::split_in_run_pace(note.pace) as i32,
    }
}

fn pace(dump: &State) -> i32 {
    super::pace(dump.total.map(|x| x.pace).unwrap_or_default()) as i32
}

fn total(dump: &State) -> Option<u32> {
    dump.total.map(|x| u32::from(x.time))
}
