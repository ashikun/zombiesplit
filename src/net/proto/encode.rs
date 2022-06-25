//! Helpers for encoding zombiesplit models to protobuf equivalents.

pub mod action;
pub mod attempt;
pub mod comparison;
pub mod dump;
pub mod event;
pub mod timing;

use super::super::{
    super::model::{game::category, session},
    metadata,
};

/// Encodes a server info dump into a protobuf response.
///
/// # Errors
///
/// None as of yet, but this may change in future.
pub fn server_info(server: &metadata::Server) -> Result<super::ServerInfoResponse> {
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

/// Encodes `pop_index` as a protobuf pop type.
/// Fails with `Missing` if `pop_index` doesn't correspond to a valid pop type.
fn pop(pop: session::action::Pop) -> i32 {
    (match pop {
        session::action::Pop::One => super::Pop::One,
        session::action::Pop::All => super::Pop::All,
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
