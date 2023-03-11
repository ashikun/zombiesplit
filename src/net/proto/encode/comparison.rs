//! Encoding logic for comparisons.

use super::super::{
    super::super::model::{short, timing},
    dump_response,
};
use std::collections::HashMap;

/// Encodes a comparison in its protobuf format.
pub fn encode(cmp: &timing::comparison::Comparison) -> dump_response::Comparison {
    dump_response::Comparison {
        run: Some(run(&cmp.run)),
        splits: segments(&cmp.splits),
    }
}

fn run(run: &timing::comparison::Run) -> dump_response::comparison::Run {
    dump_response::comparison::Run {
        sum_of_best: run.sum_of_best.map(u32::from),
        total_in_pb_run: run.total_in_pb_run.map(u32::from),
    }
}

fn segments(
    splits: &short::Map<timing::comparison::Segment>,
) -> HashMap<String, dump_response::comparison::Segment> {
    splits
        .iter()
        .map(|(sid, sp)| (sid.to_string(), segment(sp)))
        .collect()
}

fn segment(split: &timing::comparison::Segment) -> dump_response::comparison::Segment {
    dump_response::comparison::Segment {
        in_pb_run: Some(super::timing::aggregate(&split.in_pb_run)),
        split_pb: super::timing::time(&split.split_pb),
    }
}
