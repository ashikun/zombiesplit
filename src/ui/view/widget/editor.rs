/*! Logic for drawing the split editor.

The split editor isn't a [Widget] per se, as it lays on top of a split widget.
*/

use super::super::{
    super::{presenter::state, Result},
    gfx::{colour, font, metrics, position::Position, render::Renderer},
};
use crate::model::time::position::Name;

/// Calculates the size of an editor rectangle, given a text sizer.
#[must_use]
pub fn size(t: &impl font::metrics::TextSizer) -> metrics::Size {
    metrics::Size::from_i32s(t.span_w_str(PLACEHOLDER), t.span_h(1))
}

/// The editor sub-widget, borrowing renderer and editor state.
pub struct Editor<'r, 's, R> {
    /// The renderer region being targeted.
    pub r: &'r mut R,

    /// The editor state being displayed.
    pub state: &'s state::Editor,
}

const PLACEHOLDER: &str = "--'--\"---";

impl<'r, 's, R: Renderer> Editor<'r, 's, R> {
    /// Draws the foreground of any editor required by the current state.
    pub fn render(&mut self) -> Result<()> {
        self.draw_base()?;

        self.draw_field(Name::Minutes, &self.state.mins)?;
        self.draw_field(Name::Seconds, &self.state.secs)?;
        self.draw_field(Name::Milliseconds, &self.state.msecs)
    }

    fn draw_base(&mut self) -> Result<()> {
        self.r
            .set_pos(Position::top_right(self.r.span_w_str(PLACEHOLDER), 0));
        self.r.set_font(font::Id::Normal);
        self.reset_colours();
        self.fill_bg(metrics::conv::sat_i32(PLACEHOLDER.len()))?;
        self.r.put_str(PLACEHOLDER)?;
        Ok(())
    }

    fn draw_field(&mut self, pos: Name, value: &str) -> Result<()> {
        let is_editing = self.is_editing_field(pos);
        let num_chars = metrics::conv::sat_i32(value.len());

        // Visually distinguish the currently-edited editor.
        if is_editing {
            self.prepare_field_editor();
            self.fill_bg(num_chars)?;
        }

        self.r.put_str(value)?;
        self.r
            .set_pos(Position::rel_chars(self.r, num_chars + 1, 0));

        if is_editing {
            self.reset_colours();
        }

        Ok(())
    }

    fn is_editing_field(&self, pos: Name) -> bool {
        Some(pos) == self.state.field
    }

    /// Instructs the renderer to change colours to the field editor, and fill
    /// a rectangle for the field editor to sit in.
    ///
    /// The foreground and background colour will need resetting afterwards.
    fn prepare_field_editor(&mut self) {
        self.r
            .set_colours(colour::fg::Id::FieldEditor, colour::bg::Id::FieldEditor);
    }

    /// Resets the renderer's colours to the standard editor ones.
    fn reset_colours(&mut self) {
        self.r
            .set_colours(colour::fg::Id::Editor, colour::bg::Id::Editor);
    }

    /// Fills a background of `num_chars` width relative to the current position.
    fn fill_bg(&mut self, num_chars: i32) -> Result<()> {
        let size = self.r.text_size(num_chars, 1);
        self.r.fill(
            metrics::Rect {
                top_left: metrics::Point { x: 0, y: 0 },
                size,
            }
            .grow(1),
        )
    }
}
