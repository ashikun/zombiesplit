//! Font sets, config, and identifiers for looking up config from a set.

use super::metrics::Metrics;
use serde::{Deserialize, Serialize};

/// A font configuration set.
#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    /// The small font, used for attempt counts.
    pub small: Config,
    /// The normal font.
    pub normal: Config,
    /// The large font, used for titles and totals.
    pub large: Config,
}

impl Set {
    /// Gets the configuration for a particular font.
    #[must_use]
    pub fn get(&self, id: Id) -> &Config {
        match id {
            Id::Small => &self.small,
            Id::Normal => &self.normal,
            Id::Large => &self.large,
        }
    }
}

/// A key in the font manager's lookup table.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Id {
    /// Small font.
    Small,
    /// Normal font.
    Normal,
    /// Large font.
    Large,
}

// A font specification (ID and colour).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Spec {
    /// The identifier of the font.
    pub id: Id,
    /// The colour key for the font.
    pub colour: super::super::colour::fg::Id,
}

/// A font configuration.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The font path.
    pub path: String,
    /// The font metrics.
    pub metrics: Metrics,
}
