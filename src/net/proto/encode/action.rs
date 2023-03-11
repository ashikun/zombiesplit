//! Encodes zombiesplit action information into protobufs.

use super::{
    super::super::super::model::{session, timing::time},
    Result,
};

/// Encodes a push action.
///
/// # Errors
///
/// Fails if we can't fit the split index into a 64-bit integer.
pub fn push(index: usize, time: time::Time) -> Result<super::super::PushRequest> {
    Ok(super::super::PushRequest {
        index: super::try_from_range(index)?,
        time: time.into_millis(),
    })
}

/// Encodes a pop action.
///
/// # Errors
///
/// Fails if we can't fit the split index into a 64-bit integer.
pub fn pop(index: usize, ty: session::action::Pop) -> Result<super::super::PopRequest> {
    Ok(super::super::PopRequest {
        index: super::try_from_range(index)?,
        r#type: super::pop(ty),
    })
}
