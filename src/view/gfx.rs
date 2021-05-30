//! Graphics rendering.

pub mod colour;
mod header;
pub mod metrics; // for now
mod position;
pub mod render;
mod split;
mod widget;

use crate::{
    model::time,
    presenter::{
        editor::{self, Editor},
        Presenter,
    },
};

use self::colour::Key;

use super::error::{Error, Result};

use position::{Position, X, Y};
use render::Renderer;
use widget::Widget;

pub struct Core<'a> {
    renderer: render::Window<'a>,
    widgets: Vec<Box<dyn widget::Widget>>,
}

impl<'a> Core<'a> {
    /// Creates a new graphics core.
    #[must_use]
    pub fn new(renderer: render::Window<'a>) -> Self {
        Self {
            renderer,
            widgets: make_widgets(metrics::WINDOW),
        }
    }

    /// Redraws the user interface.
    ///
    /// # Errors
    ///
    /// Returns an error if SDL fails to redraw the screen.
    pub fn redraw(&mut self, state: &Presenter) -> Result<()> {
        self.renderer.clear();

        for w in &mut self.widgets {
            w.render(&mut self.renderer, state)?;
        }

        if let Some(ref editor) = state.editor() {
            self.draw_editor(editor)?;
        }

        self.renderer.present();

        Ok(())
    }

    /// Draws any editor required by the current state.
    fn draw_editor(&mut self, editor: &Editor) -> Result<()> {
        self.draw_editor_time(editor)?;
        if let Some(ref f) = editor.field {
            self.draw_editor_field(f)?;
        };
        Ok(())
    }

    fn draw_editor_time(&mut self, editor: &Editor) -> Result<()> {
        self.move_to_editor();
        self.renderer.set_font(render::FontId::Normal(Key::Editor));
        self.renderer.put_str(&split::time_str(editor.time))
    }

    fn move_to_editor(&mut self) {
        // TODO(@MattWindsor91): fix editor position.
        self.renderer.set_pos(Position {
            x: X::Right(0),
            y: Y::Bottom(0),
        });
        self.renderer.move_chars(-metrics::TIME_CHARS, -1);
    }

    fn draw_editor_field(&mut self, field: &editor::Field) -> Result<()> {
        self.renderer
            .move_chars(field_char_offset(field.position()), 0);
        self.renderer
            .set_font(render::FontId::Normal(Key::FieldEditor));
        self.renderer.put_str(&field.to_string())
    }
}

fn field_char_offset(field: time::position::Name) -> i32 {
    match field {
        // Hours not supported yet.
        time::position::Name::Hours | time::position::Name::Minutes => 0,
        time::position::Name::Seconds => 3,
        time::position::Name::Milliseconds => 6,
    }
}

fn make_widgets(wmetrics: metrics::Window) -> Vec<Box<dyn Widget>> {
    vec![make_splits(wmetrics), make_header(wmetrics)]
}

fn make_splits(wmetrics: metrics::Window) -> Box<dyn Widget> {
    Box::new(split::Widget::new(
        wmetrics.splits_rect(),
        metrics::sat_i32(wmetrics.split_h),
    ))
}

fn make_header(wmetrics: metrics::Window) -> Box<dyn Widget> {
    Box::new(header::Widget {
        rect: wmetrics.header_rect(),
    })
}

/// Makes a zombiesplit window.
///
/// # Errors
///
/// Returns an error if SDL fails to make the window.
pub fn make_window(video: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window> {
    let window = video
        .window("zombiesplit", metrics::WINDOW.win_w, metrics::WINDOW.win_h)
        .position_centered()
        .build()
        .map_err(Error::Window)?;
    Ok(window)
}
