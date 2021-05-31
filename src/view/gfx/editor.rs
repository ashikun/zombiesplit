//! Logic for drawing the split editor.

use super::{
    colour, font, metrics,
    position::{Position, X, Y},
    render::{Region, Renderer},
    split, widget,
};
use crate::{
    model::time,
    presenter::{
        editor::{Editor, Field},
        Presenter,
    },
    view::error::Result,
};

/// The editor widget.
pub struct Widget {
    /// The bounding box for the header widget, to be offset by the split
    /// position.
    rect: metrics::Rect,
}

impl widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        if let Some(e) = p.editor() {
            let mut rect = self.rect;
            rect.y += metrics::sat_i32(rect.h) * metrics::sat_i32(e.cur.position());
            let mut r = Region::new(r, rect);
            draw_editor(&mut r, e)?;
        }
        Ok(())
    }
}

impl Widget {
    /// Creates a new editor widget given the baseline rect.
    pub fn new(rect: metrics::Rect) -> Self {
        Self { rect }
    }
}

/// Draws any editor required by the current state.
fn draw_editor(r: &mut dyn Renderer, editor: &Editor) -> Result<()> {
    // Every part of the editor uses the normal font.
    r.set_font(font::Id::Normal)?;

    draw_time(r, editor)?;
    if let Some(ref f) = editor.field {
        draw_field(r, f)?;
    };
    Ok(())
}

fn draw_time(r: &mut dyn Renderer, editor: &Editor) -> Result<()> {
    move_to_editor(r);
    r.set_fg_colour(colour::Key::Editor);
    r.put_str(&split::time_str(editor.time))
}

fn move_to_editor(r: &mut dyn Renderer) {
    // TODO(@MattWindsor91): fix editor position.
    r.set_pos(Position {
        x: X::Right(0),
        y: Y::Top(0),
    });
    r.move_chars(-metrics::TIME_CHARS, 0);
}

fn draw_field(r: &mut dyn Renderer, field: &Field) -> Result<()> {
    // Position floats above main editor.
    r.move_chars(field_char_offset(field.position()), 0);
    r.set_fg_colour(colour::Key::FieldEditor);
    r.put_str(&field.to_string())
}

fn field_char_offset(field: time::position::Name) -> i32 {
    match field {
        // Hours not supported yet.
        time::position::Name::Hours | time::position::Name::Minutes => 0,
        time::position::Name::Seconds => 3,
        time::position::Name::Milliseconds => 6,
    }
}
