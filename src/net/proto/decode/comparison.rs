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
        splits: splits(&cmp.splits)?,
    })
}

fn run(run: &dump_response::comparison::Run) -> Result<timing::comparison::Run> {
    Ok(timing::comparison::Run {
        total_in_pb_run: run.total_in_pb_run.map(super::time).transpose()?,
        sum_of_best: run.sum_of_best.map(super::time).transpose()?,
    })
}

fn splits(
    splits: &HashMap<String, dump_response::comparison::Split>,
) -> Result<short::Map<timing::comparison::Split>> {
    splits
        .iter()
        .map(|(sid, sp)| Ok((short::Name::from(sid), split(sp)?)))
        .collect()
}

fn split(split: &dump_response::comparison::Split) -> Result<timing::comparison::Split> {
    Ok(timing::comparison::Split {
        in_pb_run: split
            .in_pb_run
            .as_ref()
            .map(aggregate)
            .transpose()?
            .unwrap_or_default(),
        split_pb: super::time(split.split_pb)?,
    })
}

fn aggregate(agg: &dump_response::comparison::split::Aggregate) -> Result<timing::aggregate::Set> {
    Ok(timing::aggregate::Set {
        split: super::time(agg.split)?,
        cumulative: super::time(agg.cumulative)?,
    })
}
