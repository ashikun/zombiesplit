//! Layout configuration for the view.
//!
//! This module contains configuration (and default configuration) for laying out the UI and its
//! various widgets.  It is passed to the widgets when determining their layout.

use serde::{Deserialize, Serialize};
use ugly::metrics;

use super::super::{
    super::{super::model::timing::time, presenter::state::footer},
    gfx::font,
};

/// Layout configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Layout {
    /// Window metrics.
    pub window: Window,
    /// Default format for times.
    pub time: time::Format,
    /// Layout information for widgets.
    pub widgets: WidgetSet,
}

/// Window metrics.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Window {
    /// Width and height of the window.
    pub size: metrics::Size,
    /// Padding used throughout the window.
    pub padding: i32,
}

/// Layout configuration for widgets.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct WidgetSet {
    /// Layout configuration for footers.
    pub footer: Footer,
}

/// Layout configuration for footers.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Footer {
    /// The rows configured on this footer.
    pub rows: Vec<Row>,
}

/// The default set of rows displayed in the footer.
impl Default for Footer {
    fn default() -> Self {
        Self {
            rows: vec![
                Row {
                    contents: footer::RowType::Total,
                    font: font::Id::Large,
                },
                Row {
                    contents: footer::RowType::Comparison,
                    font: font::Id::Large,
                },
                Row {
                    contents: footer::RowType::SumOfBest,
                    font: font::Id::Medium,
                },
            ],
        }
    }
}

/// Configuration for a row in the footer.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    /// The type of data being shown in this row.
    pub contents: footer::RowType,
    /// The font to use for the time.
    pub font: font::Id,
}
