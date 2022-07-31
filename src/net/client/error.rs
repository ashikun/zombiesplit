//! Error types for the client.
use thiserror::Error;

/// Type of client errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Couldn't convert a string into a client URI.
    #[error("couldn't parse URI")]
    BadUri(#[from] http::uri::InvalidUri),
    /// A general I/O error.
    #[error("i/o error")]
    Io(#[from] std::io::Error),
    /// A gRPC transport error.
    #[error("transport error")]
    Transport(#[from] tonic::transport::Error),
    /// An error from the gRPC server.
    #[error("error requesting server action")]
    Server(#[from] tonic::Status),
    /// A client-side decoding error.
    #[error("error decoding response from server")]
    Decode(#[from] super::proto::decode::Error),
    /// Couldn't cancel the server's observation loop.
    #[error("couldn't cancel the observer")]
    ObserverCancelFail,
}

/// Shorthand for results over [Error].
pub type Result<T> = std::result::Result<T, Error>;
