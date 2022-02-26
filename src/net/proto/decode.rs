//! Helpers for decoding protobuf representations of zombiesplit models.

use crate::model::timing::time;

/// Shorthand for decoding errors.
///
/// These invariably result in gRPC status codes on failure.
pub type Result<T> = std::result::Result<T, tonic::Status>;

/// Decodes a protobuf representation of a split index into a model representation.
///
/// This is a straightforward integer bit width change.
///
/// # Errors
///
/// Fails with `out_of_range` if the index is too large (which may happen on eg. 32-bit systems).
pub fn split_index(index: u64) -> std::result::Result<usize, tonic::Status> {
    usize::try_from(index).map_err(|e| tonic::Status::out_of_range(e.to_string()))
}

/// Decodes a timestamp into a time.
///
/// # Errors
///
/// Fails with `out_of_range` if the timestamp is too large to represent a valid time, and
/// `invalid_argument` if there is any other error in decoding the time.
pub fn time(stamp: u32) -> std::result::Result<time::Time, tonic::Status> {
    time::Time::try_from(stamp).map_err(adapt_time_error)
}

fn adapt_time_error(err: time::Error) -> tonic::Status {
    match err {
        time::Error::MsecOverflow(k) => {
            tonic::Status::out_of_range(format!("millisecond value {k} too large"))
        }
        _ => tonic::Status::invalid_argument(format!("invalid time: {err}")),
    }
}
