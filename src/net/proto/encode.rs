//! Helpers for encoding zombiesplit models to protobuf equivalents.

pub mod attempt;
pub mod comparison;
pub mod event;

use super::super::{
    super::model::{
        attempt::{action, session},
        game::category,
        timing,
    },
    dump,
};

/// Encodes a server info dump into a protobuf response.
///
/// # Errors
///
/// None as of yet, but this may change in future.
pub fn server_info(server: &dump::Server) -> Result<super::ServerInfoResponse> {
    Ok(super::ServerInfoResponse {
        ident: server.ident.clone(),
        version: Some(version(&server.version)),
    })
}

fn version(version: &semver::Version) -> super::server_info_response::Version {
    super::server_info_response::Version {
        major: version.major,
        minor: version.minor,
        patch: version.patch,
    }
}

/// Encodes a dump into a protobuf response.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn dump(dump: &session::State) -> Result<super::DumpResponse> {
    Ok(super::DumpResponse {
        attempt: Some(attempt::encode(&dump.run)?),
        comparison: Some(comparison::encode(&dump.comparison)),
    })
}

/// Encodes attempt information into its protobuf form.
///
/// # Errors
///
/// Fails with `out_of_range` if the attempt counts cannot be stored as 64-bit integers.
pub fn attempt_info(attempt: &category::AttemptInfo) -> Result<super::AttemptInfo> {
    Ok(super::AttemptInfo {
        total: try_from_range(attempt.total)?,
        completed: try_from_range(attempt.completed)?,
    })
}

fn pace(pace: timing::comparison::Pace) -> super::Pace {
    match pace {
        timing::comparison::Pace::Inconclusive => super::Pace::None,
        timing::comparison::Pace::Behind => super::Pace::Behind,
        timing::comparison::Pace::Ahead => super::Pace::Ahead,
        timing::comparison::Pace::PersonalBest => super::Pace::PersonalBest,
    }
}

/// Encodes a push action.
///
/// # Errors
///
/// Fails if we can't fit the split index into a 64-bit integer.
pub fn push_action(index: usize, time: timing::Time) -> Result<super::PushRequest> {
    Ok(super::PushRequest {
        index: try_from_range(index)?,
        time: u32::from(time),
    })
}

/// Encodes a pop action.
///
/// # Errors
///
/// Fails if we can't fit the split index into a 64-bit integer.
pub fn pop_action(index: usize, ty: action::Pop) -> Result<super::PopRequest> {
    Ok(super::PopRequest {
        index: try_from_range(index)?,
        r#type: pop(ty),
    })
}

/// Encodes `pop_index` as a protobuf pop type.
/// Fails with `Missing` if `pop_index` doesn't correspond to a valid pop type.
fn pop(pop: action::Pop) -> i32 {
    (match pop {
        action::Pop::One => super::Pop::One,
        action::Pop::All => super::Pop::All,
    }) as i32
}

fn try_from_range<E: ToString, X, Y: TryFrom<X, Error = E>>(
    x: X,
) -> std::result::Result<Y, tonic::Status> {
    Y::try_from(x).map_err(|e| tonic::Status::out_of_range(e.to_string()))
}

/// Shorthand for encoding errors.
///
/// These invariably result in gRPC status codes on failure.
pub type Result<T> = std::result::Result<T, tonic::Status>;
