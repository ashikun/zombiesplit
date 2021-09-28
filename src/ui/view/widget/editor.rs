/*! Logic for drawing the split editor.

The split editor isn't a [Widget] per se, as it lays on top of a split widget.
*/

use super::super::{
    super::{presenter::state, Result},
    gfx::{
        colour, font,
        metrics::{self, Anchor, Rect},
        render::Renderer,
    },
};
use crate::model::time::position::Name;

/// Calculates the size of an editor rectangle, given a text sizer.
#[must_use]
pub fn size(t: &(impl font::metrics::TextSizer + ?Sized)) -> metrics::Size {
    metrics::Size::from_i32s(t.span_w_str(PLACEHOLDER), t.span_h(1))
}

/// The editor sub-widget, borrowing renderer and editor state.
pub struct Editor {
    /// The bounding box of the editor.
    pub rect: Rect,
}

const PLACEHOLDER: &str = "--'--\"---";

impl super::Widget<state::Editor> for Editor {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.rect = ctx.bounds;
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::Editor) -> Result<()> {
        self.draw_base(r)?;

        // TODO(@MattWindsor91): this is temporary.
        let mut pos = self.rect.point(0, 0, Anchor::TOP_LEFT);
        for field in [Name::Minutes, Name::Seconds, Name::Milliseconds] {
            r.set_pos(pos);
            pos.offset_mut(r.span_w(2), 0);
            draw_field(r, s.field(field), s.field == Some(field))?;
        }

        Ok(())
    }
}

impl Editor {
    fn draw_base(&self, r: &mut dyn Renderer) -> Result<()> {
        r.set_pos(self.rect.top_left);
        r.set_font(font::Id::Normal);
        reset_colours(r);
        fill_bg(r, metrics::conv::sat_i32(PLACEHOLDER.len()))?;
        r.put_str(PLACEHOLDER)?;
        Ok(())
    }
}

fn draw_field(r: &mut dyn Renderer, value: &str, is_editing: bool) -> Result<()> {
    let num_chars = metrics::conv::sat_i32(value.len());

    // Visually distinguish the currently-edited editor.
    if is_editing {
        prepare_field_editor(r);
        fill_bg(r, num_chars)?;
    }

    r.put_str(value)?;

    if is_editing {
        reset_colours(r);
    }

    Ok(())
}

/// Fills a background of `num_chars` width relative to the current position.
fn fill_bg(r: &mut dyn Renderer, num_chars: i32) -> Result<()> {
    let size = r.text_size(num_chars, 1);
    r.fill(
        metrics::Rect {
            top_left: metrics::Point { x: 0, y: 0 },
            size,
        }
        .grow(1),
    )
}

/// Instructs the renderer to change colours to the field editor, and fill
/// a rectangle for the field editor to sit in.
///
/// The foreground and background colour will need resetting afterwards.
fn prepare_field_editor(r: &mut dyn Renderer) {
    r.set_colours(colour::fg::Id::FieldEditor, colour::bg::Id::FieldEditor);
}

/// Resets the renderer's colours to the standard editor ones.
fn reset_colours(r: &mut dyn Renderer) {
    r.set_colours(colour::fg::Id::Editor, colour::bg::Id::Editor);
}
