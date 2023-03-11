//! Decoding logic for top-level session dumps.

use super::{
    super::{
        super::super::model::{session, short, timing},
        dump_response, DumpResponse,
    },
    attempt, comparison, Missing, Result,
};
use std::collections::HashMap;

/// Decodes a protobuf representation of a dump.
///
/// # Errors
///
/// Fails if the attempt counts cannot be stored as `usize`, or if anything is missing from the
/// dump that we expect to see.
pub fn dump(dump: &DumpResponse) -> Result<session::State> {
    // TODO(@MattWindsor91): carry aggregates through protobufs.
    Ok(session::State {
        attempt: attempt::decode(Missing::Attempt.require(dump.attempt.as_ref())?)?,
        comparison: dump
            .comparison
            .as_ref()
            .map(comparison::decode)
            .transpose()?
            .unwrap_or_default(),
        notes: notes(&dump.notes)?,
        total: total(dump)?,
    })
}

fn notes(
    notes: &HashMap<String, dump_response::SplitNote>,
) -> Result<short::Map<session::state::SplitNote>> {
    notes
        .iter()
        .map(|(s, n)| Ok((short::Name::from(&s), note(n)?)))
        .collect()
}

fn note(note: &dump_response::SplitNote) -> Result<session::state::SplitNote> {
    let aggregates = note
        .aggregate
        .as_ref()
        .map(super::timing::aggregate)
        .unwrap_or_default();
    let delta = note
        .delta
        .as_ref()
        .map(super::timing::split_delta)
        .transpose()?;
    Ok(session::state::SplitNote { aggregates, delta })
}

fn total(dump: &DumpResponse) -> Result<Option<timing::comparison::delta::Time>> {
    dump.total.as_ref().map(super::timing::total).transpose()
}
