//! Logic for drawing the split editor.

use super::{
    super::{
        super::presenter::{
            editor::{Editor, Field},
            Presenter,
        },
        error::Result,
    },
    colour, font, metrics,
    position::{Position, X},
    render::{Region, Renderer},
    split, widget,
};
use crate::model::time;

/// The editor widget.
pub struct Widget {
    /// The bounding box for the header widget, to be offset by the split
    /// position.
    rect: metrics::Rect,
}

impl widget::Widget for Widget {
    fn render(&mut self, r: &mut dyn Renderer, p: &Presenter) -> Result<()> {
        if let Some(e) = p.editor() {
            self.draw_editor(e, r)?;
        }
        Ok(())
    }
}

impl Widget {
    /// Creates a new editor widget given the baseline rect.
    pub fn new(rect: metrics::Rect) -> Self {
        Self { rect }
    }

    /// Draws any editor required by the current state.
    fn draw_editor(&mut self, e: &Editor, r: &mut dyn Renderer) -> Result<()> {
        let mut rect = self.rect;
        rect.y += metrics::sat_i32(rect.size.h) * metrics::sat_i32(e.cur.position());

        // TODO(@MattWindsor91): don't hardcode this padding factor

        let mut r = Region::new(r, rect);
        draw_editor(&mut r, e)?;
        Ok(())
    }
}

/// Width of the editor in pixels.
const EDITOR_WIDTH: i32 = 9;

/// Draws the foreground of any editor required by the current state.
fn draw_editor(r: &mut dyn Renderer, editor: &Editor) -> Result<()> {
    let size = r.text_size(EDITOR_WIDTH, 1);

    r.set_pos(Position::top_right(metrics::sat_i32(size.w), 0));
    r.set_font(font::Id::Normal);
    r.set_fg_colour(colour::fg::Id::Editor);
    r.set_bg_colour(colour::bg::Id::Editor);

    r.fill(metrics::Rect { x: 0, y: 0, size }.grow(1))?;

    draw_editor_fg(r, editor)
}

/// Draws the foreground of any editor required by the current state.
fn draw_editor_fg(r: &mut dyn Renderer, editor: &Editor) -> Result<()> {
    // Every part of the editor uses the normal font.
    r.set_font(font::Id::Normal);
    r.set_pos(Position::top_right(0, 0));

    draw_time(r, editor)?;
    if let Some(ref f) = editor.field {
        draw_field(r, f)?;
    };
    Ok(())
}

fn draw_time(r: &mut dyn Renderer, editor: &Editor) -> Result<()> {
    r.set_fg_colour(colour::fg::Id::Editor);
    r.put_str_r(&split::time_str(editor.time))
}

fn draw_field(r: &mut dyn Renderer, field: &Field) -> Result<()> {
    let pos = field.position();

    // Position floats above main editor.
    r.set_pos(Position::x(X::Right(
        r.span_w(EDITOR_WIDTH - field_char_offset(pos)),
    )));
    r.set_fg_colour(colour::fg::Id::FieldEditor);
    r.set_bg_colour(colour::bg::Id::FieldEditor);

    let size = r.text_size(field_char_width(pos), 1);
    r.fill(metrics::Rect { x: 0, y: 0, size }.grow(1))?;

    r.put_str(&field.to_string())
}

fn field_char_width(field: time::position::Name) -> i32 {
    match field {
        time::position::Name::Milliseconds => 3,
        time::position::Name::Seconds | time::position::Name::Minutes => 2,
        time::position::Name::Hours => unimplemented!(),
    }
}

/// Gets the field left X-offset, in chars.
fn field_char_offset(field: time::position::Name) -> i32 {
    match field {
        time::position::Name::Minutes => 0,
        time::position::Name::Seconds => 3,
        time::position::Name::Milliseconds => 6,
        time::position::Name::Hours => unimplemented!(),
    }
}
