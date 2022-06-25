//! Top-level encoding for state dumps.

use super::{
    super::{
        super::super::model::{session, short},
        dump_response, DumpResponse,
    },
    attempt, comparison, Result,
};
use std::collections::HashMap;

/// Encodes a dump into a protobuf response.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn encode(dump: &session::State) -> Result<DumpResponse> {
    Ok(DumpResponse {
        attempt: Some(attempt::encode(&dump.attempt)?),
        comparison: Some(comparison::encode(&dump.comparison)),
        notes: notes(&dump.notes),
        total: dump.total.map(super::timing::total),
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
        aggregate: Some(super::timing::aggregate(&note.aggregates)),
        delta: note.delta.as_ref().map(super::timing::split_delta),
    }
}
