//! Dumps of server state, to be consumed by the client.

/// Dump of server metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Server {
    /// Server identifier.
    pub ident: String,
    /// Server protocol semantic version.
    pub version: semver::Version,
}
