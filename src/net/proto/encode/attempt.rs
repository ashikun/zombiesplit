//! Encoding logic for run attempts.

use super::{
    super::{
        super::super::model::{game::category, session},
        dump_response,
    },
    Result,
};
use itertools::Itertools;

/// Encodes a comparison into its protobuf format.
///
/// # Errors
///
/// Fails if any of the indices in the attempt information are too large to store as `u64`.
pub fn encode(run: &session::Attempt) -> Result<dump_response::Attempt> {
    Ok(dump_response::Attempt {
        target: Some(target(&run.category)),
        attempt_info: Some(super::attempt_info(&run.info)?),
        splits: splits(&run.splits),
    })
}

fn target(info: &category::Target) -> dump_response::attempt::Target {
    dump_response::attempt::Target {
        category_name: info.category.clone(),
        game_name: info.game.clone(),
        category_sid: info.short.category.to_string(),
        game_sid: info.short.game.to_string(),
    }
}

fn splits(splits: &session::split::Set) -> Vec<dump_response::attempt::Split> {
    splits.iter().map(split).collect_vec()
}

fn split(split: &session::Split) -> dump_response::attempt::Split {
    dump_response::attempt::Split {
        sid: split.info.short.to_string(),
        name: split.info.name.clone(),
        times: times(split),
        nickname: split.info.nickname.clone(),
    }
}

fn times(split: &session::Split) -> Vec<u32> {
    split.times.iter().map(|x| u32::from(*x)).collect_vec()
}
