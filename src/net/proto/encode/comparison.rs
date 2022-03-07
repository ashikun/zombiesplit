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
        splits: splits(&cmp.splits),
    }
}

fn run(run: &timing::comparison::Run) -> dump_response::comparison::Run {
    dump_response::comparison::Run {
        sum_of_best: run.sum_of_best.map(u32::from),
        total_in_pb_run: run.total_in_pb_run.map(u32::from),
    }
}

fn splits(
    splits: &short::Map<timing::comparison::Split>,
) -> HashMap<String, dump_response::comparison::Split> {
    splits
        .iter()
        .map(|(sid, sp)| (sid.to_string(), split(sp)))
        .collect()
}

fn split(split: &timing::comparison::Split) -> dump_response::comparison::Split {
    dump_response::comparison::Split {
        in_pb_run: Some(aggregate(&split.in_pb_run)),
        split_pb: u32::from(split.split_pb),
    }
}

fn aggregate(agg: &timing::aggregate::Set) -> dump_response::comparison::split::Aggregate {
    dump_response::comparison::split::Aggregate {
        split: u32::from(agg.split),
        cumulative: u32::from(agg.cumulative),
    }
}
