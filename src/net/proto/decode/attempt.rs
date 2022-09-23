//! Decoding logic for run attempts.

use super::{
    super::{
        super::super::model::{game, session, short, timing::time::human},
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
pub fn decode(run: &dump_response::Attempt) -> Result<session::Attempt> {
    Ok(session::Attempt {
        category: game_category(Missing::AttemptInfo.require(run.game_category.as_ref())?),
        info: run
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

fn splits(splits: &[dump_response::attempt::Split]) -> Result<session::split::Set> {
    splits.iter().map(split).collect()
}

fn split(split: &dump_response::attempt::Split) -> Result<session::split::Split> {
    Ok(session::split::Split {
        info: game::Split {
            short: short::Name::from(&split.sid),
            name: split.name.clone(),
            nickname: split.nickname.clone(),
        },
        times: times(split)?,
    })
}

fn times(split: &dump_response::attempt::Split) -> Result<Vec<human::Time>> {
    split
        .times
        .iter()
        .map(|x| super::timing::time(*x))
        .collect()
}
