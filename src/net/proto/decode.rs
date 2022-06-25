//! Helpers for decoding protobuf representations of zombiesplit models.

pub mod action;
pub mod attempt;
pub mod comparison;
pub mod dump;
pub mod error;
pub mod event;
pub mod timing;

use super::super::super::model::{game::category, session};
pub use error::{Error, Missing, Result, Unknown};

/// Decodes a protobuf representation of attempt information into its model form.
///
/// # Errors
///
/// Fails if the attempt counts cannot be stored as `usize`.
pub fn attempt_info(attempt: &super::AttemptInfo) -> Result<category::AttemptInfo> {
    Ok(category::AttemptInfo {
        total: usize::try_from(attempt.total)?,
        completed: usize::try_from(attempt.completed)?,
    })
}

/// Tries to interpret `pop_index` as a protobuf pop type, and decode it into the model form.
///
/// # Errors
///
/// Fails with `Missing` if `pop_index` doesn't correspond to a valid pop type.
fn pop(pop_index: i32) -> Result<session::action::Pop> {
    Ok(
        match Unknown::Pop.require(super::Pop::from_i32(pop_index))? {
            super::Pop::One => session::action::Pop::One,
            super::Pop::All => session::action::Pop::All,
        },
    )
}

/// Decodes a protobuf representation of a split index into a model representation.
///
/// This is a straightforward integer bit width change.
///
/// # Errors
///
/// Fails with `out_of_range` if the index is too large (which may happen on eg. 32-bit systems).
pub fn split_index(index: u64) -> Result<usize> {
    Ok(usize::try_from(index)?)
}
