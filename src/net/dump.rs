//! Dumps of server state, to be consumed by the client.

/// Full server state dump.
#[derive(Clone, Debug)]
pub struct Dump {
    /// Server metadata.
    pub server: Server,
    /// Clone of the current run located in the session.
    pub run: crate::model::attempt::Run,
}

/// Dump of server metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Server {
    /// Server identifier.
    pub ident: String,
    /// Server protocol semantic version.
    pub version: semver::Version,
}
