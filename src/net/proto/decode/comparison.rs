//! Decoding logic for comparisons.

use super::{
    super::{
        super::super::model::{short, timing},
        dump_response,
    },
    error::Result,
};
use std::collections::HashMap;

/// Decodes a comparison.
pub fn decode(cmp: &dump_response::Comparison) -> Result<timing::comparison::Comparison> {
    Ok(timing::comparison::Comparison {
        run: cmp.run.as_ref().map(run).transpose()?.unwrap_or_default(),
        splits: segments(&cmp.splits)?,
    })
}

fn run(run: &dump_response::comparison::Run) -> Result<timing::comparison::Run> {
    Ok(timing::comparison::Run {
        total_in_pb_run: run
            .total_in_pb_run
            .map(timing::time::Time::from_millis)
            .transpose()?,
        sum_of_best: run
            .sum_of_best
            .map(timing::time::Time::from_millis)
            .transpose()?,
    })
}

fn segments(
    segments: &HashMap<String, dump_response::comparison::Segment>,
) -> Result<short::Map<timing::comparison::Segment>> {
    segments
        .iter()
        .map(|(sid, sp)| Ok((short::Name::from(sid), segment(sp)?)))
        .collect()
}

fn segment(segment: &dump_response::comparison::Segment) -> Result<timing::comparison::Segment> {
    Ok(timing::comparison::Segment {
        in_pb_run: segment
            .in_pb_run
            .as_ref()
            .map(super::timing::aggregate)
            .transpose()?
            .unwrap_or_default(),
        split_pb: timing::time::Time::from_millis(segment.split_pb)?,
    })
}
