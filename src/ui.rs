//! The zombiesplit user interface.
pub mod error;
pub mod gfx;

pub use error::{Error, Result};
use sdl2::{keyboard::Keycode, rect::Point};

use self::gfx::Renderer;
use crate::model::run;

/// The UI core.
pub struct Core {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,

    state: State,
}

impl Core {
    /// Creates a new UI core.
    pub fn new(run: run::Run) -> error::Result<Self> {
        let sdl = sdl2::init().map_err(Error::SdlInit)?;
        let video = sdl.video().map_err(Error::SdlInit)?;
        Ok(Core {
            sdl,
            video,
            state: State::new(run),
        })
    }

    /// Runs the UI loop.
    pub fn run_loop(&mut self) -> error::Result<()> {
        // TODO(@MattWindsor91): pass in something other than Game.

        let window = gfx::make_window(&self.video)?;

        let screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
        let textures = screen.texture_creator();

        let font = gfx::load_font(&textures, "font.png")?;

        let mut r = gfx::Renderer { screen, font };

        self.redraw(&mut r)?;

        let mut events = self.sdl.event_pump().map_err(Error::SdlInit)?;
        while self.state.running {
            for event in events.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit { .. } => self.state.running = false,
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => self.handle_keydown(k, &mut r)?,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_keydown(&mut self, k: Keycode, r: &mut Renderer) -> Result<()> {
        match k {
            sdl2::keyboard::Keycode::J => {
                if self.state.move_cursor_down() {
                    self.redraw(r)?
                }
            }
            sdl2::keyboard::Keycode::K => {
                if self.state.move_cursor_up() {
                    self.redraw(r)?
                }
            }
            sdl2::keyboard::Keycode::Escape => self.state.running = false,
            _ => {}
        };
        Ok(())
    }

    fn redraw(&self, r: &mut Renderer) -> Result<()> {
        r.clear();

        self.draw_splits(self.state.run.splits.iter(), r)?;

        r.present();

        Ok(())
    }

    fn draw_splits<'a>(
        &self,
        splits: impl IntoIterator<Item = &'a run::Split>,
        r: &mut Renderer,
    ) -> Result<()> {
        for (num, s) in splits.into_iter().enumerate() {
            self.draw_split(s, num, r)?;
        }
        Ok(())
    }

    fn draw_split(&self, split: &run::Split, num: usize, r: &mut Renderer) -> Result<()> {
        let tl = self.split_name_top_left(num);

        r.set_font_colour(self.split_font_colour(num));
        r.put_str(&split.name, tl)
    }

    fn split_name_top_left(&self, num: usize) -> sdl2::rect::Point {
        Point::new(4, 4 + (16 * num as i32))
    }

    fn split_font_colour(&self, num: usize) -> sdl2::pixels::Color {
        gfx::colours::SET.fg_split_text(num, self.state.cursor)
    }
}

struct State {
    /// The current split.
    cursor: usize,
    /// The current run.
    run: run::Run,
    /// Whether the UI is running.
    running: bool,
}

impl State {
    fn new(run: run::Run) -> Self {
        Self {
            cursor: 0,
            run,
            running: true,
        }
    }

    /// Moves the state cursor up.  Returns true if the cursor moved successfully.
    pub fn move_cursor_up(&mut self) -> bool {
        if self.cursor == 0 {
            false
        } else {
            self.cursor -= 1;
            true
        }
    }

    /// Moves the state cursor down.  Returns true if the cursor moved successfully.
    pub fn move_cursor_down(&mut self) -> bool {
        if self.cursor == self.run.splits.len() - 1 {
            false
        } else {
            self.cursor += 1;
            true
        }
    }
}
