//! Error types for the server.

use thiserror::Error;

/// The top-level server error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("database error")]
    Db(#[from] crate::db::Error),
    #[error("i/o error")]
    IO(#[from] std::io::Error),
    #[error("couldn't join task")]
    Join(#[from] tokio::task::JoinError),
    #[error("couldn't send action to session")]
    CannotSendAction(#[from] tokio::sync::mpsc::error::SendError<crate::model::attempt::Action>),
}

/// The top-level server result type.
pub type Result<T> = std::result::Result<T, Error>;
