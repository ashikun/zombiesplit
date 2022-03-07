//! Decoding logic for run attempts.

use super::{
    super::{
        super::super::model::{attempt, game, short, timing},
        dump_response,
    },
    error::{Missing, Result},
};

/// Decodes attempt information from its protobuf format to its model representation.
///
/// # Errors
///
/// Fails if the counts in the attempt information overflow `usize`s on this machine, or if any of
/// the times supplied for the splits are ill-formed.
pub fn decode(run: &dump_response::Attempt) -> Result<attempt::Run> {
    Ok(attempt::Run {
        metadata: game_category(Missing::AttemptInfo.require(run.game_category.as_ref())?),
        attempt: run
            .attempt_info
            .as_ref()
            .map(super::attempt_info)
            .transpose()?
            .unwrap_or_default(),
        splits: splits(&run.splits)?,
    })
}

fn game_category(info: &dump_response::attempt::GameCategory) -> game::category::Info {
    game::category::Info {
        game: info.game_name.clone(),
        category: info.category_name.clone(),
        short: game::category::ShortDescriptor::new(&info.game_sid, &info.category_sid),
    }
}

fn splits(splits: &[dump_response::attempt::Split]) -> Result<attempt::split::Set> {
    splits.iter().map(split).collect()
}

fn split(split: &dump_response::attempt::Split) -> Result<attempt::split::Split> {
    Ok(attempt::split::Split {
        info: game::Split {
            short: short::Name::from(&split.sid),
            name: split.name.clone(),
        },
        times: times(split)?,
    })
}

fn times(split: &dump_response::attempt::Split) -> Result<Vec<timing::Time>> {
    split.times.iter().map(|x| super::time(*x)).collect()
}
