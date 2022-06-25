//! Decodes protobufs into zombiesplit action information.

use super::{super::super::super::model::session, Result};

/// Decodes a push action.
///
/// # Errors
///
/// Fails if the split index or time are out of bounds.
pub fn push(request: &super::super::PushRequest) -> Result<session::Action> {
    Ok(session::Action::Push(
        super::split_index(request.index)?,
        super::timing::time(request.time)?,
    ))
}

/// Decodes a pop action.
///
/// # Errors
///
/// Fails if the split index is out of bounds, or the pop type is malformed.
pub fn pop(request: &super::super::PopRequest) -> Result<session::Action> {
    Ok(session::Action::Pop(
        super::split_index(request.index)?,
        super::pop(request.r#type)?,
    ))
}
