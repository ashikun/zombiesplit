/*! Logic for drawing the split editor.

The split editor isn't a [Widget] per se, as it lays on top of a split widget.
*/

use super::super::{
    super::presenter::state,
    gfx::{self, colour, font, metrics, render::Renderer},
    Widget,
};

use crate::model::time::position::Index;

/// The editor sub-widget, borrowing renderer and editor state.
#[derive(Default)]
pub struct Editor {
    /// Basic layout for the editor.
    layout: super::super::time::Layout,
}

/// Delegates to the underlying editor state, if there is any.
impl Widget<state::Split> for Editor {
    fn layout(&mut self, ctx: super::LayoutContext) {
        Widget::<state::Editor>::layout(self, ctx);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::Split) -> gfx::Result<()> {
        s.editor
            .as_ref()
            .map_or(Ok(()), |e| (Widget::<state::Editor>::render(self, r, e)))
    }
}

impl Widget<state::Editor> for Editor {
    fn layout(&mut self, ctx: super::LayoutContext) {
        self.layout.update(ctx);
    }

    fn render(&self, r: &mut dyn Renderer, s: &state::Editor) -> gfx::Result<()> {
        self.draw_base(r)?;

        let mut pos = self.layout.rect.top_left;
        for field in [Index::Minutes, Index::Seconds, Index::Milliseconds] {
            r.set_pos(pos);
            let metr = &r.font_metrics()[font::Id::Medium];
            // TODO(@MattWindsor91): make this less awful
            pos.offset_mut(
                metr.span_w_str(field_placeholder(field)) + metrics::conv::sat_i32(metr.pad.w),
                0,
            );
            draw_field(r, s.field(field), s.field == Some(field))?;
        }

        Ok(())
    }
}

const fn field_placeholder(f: Index) -> &'static str {
    match f {
        Index::Minutes => "  '",
        Index::Seconds => "  \"",
        _ => "",
    }
}

impl Editor {
    fn draw_base(&self, r: &mut dyn Renderer) -> gfx::Result<()> {
        r.set_pos(self.layout.rect.top_left);
        r.set_font(font::Id::Medium);
        reset_colours(r);
        fill_bg(
            r,
            metrics::conv::sat_i32(super::super::time::PLACEHOLDER.len()),
        )?;
        r.put_str(super::super::time::PLACEHOLDER)?;
        Ok(())
    }
}

fn draw_field(r: &mut dyn Renderer, value: &str, is_editing: bool) -> gfx::Result<()> {
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
fn fill_bg(r: &mut dyn Renderer, num_chars: i32) -> gfx::Result<()> {
    let size = r.font_metrics()[font::Id::Medium].text_size(num_chars, 1);
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
