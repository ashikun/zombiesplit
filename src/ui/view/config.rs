//! View configuration.

pub mod layout;
pub mod theme;

pub use layout::Layout;
pub use theme::Theme;

/// Top-level UI configuration.
#[derive(Clone, Debug)]
pub struct Config {
    /// Theme configuration.
    pub theme: Theme,
    /// Layout configuration.
    pub layout: Layout,
}
