//! The zombiesplit user interface.
pub mod error;
pub mod gfx;

pub use error::{Error, Result};
use sdl2::rect::Point;

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
        while self.state.is_running() {
            events
                .poll_iter()
                .filter_map(Event::from_sdl)
                .try_for_each(|e| self.handle_event(e, &mut r))?;
        }

        Ok(())
    }

    fn handle_event(&mut self, e: Event, r: &mut Renderer) -> Result<()> {
        let need_redraw = self.state.handle_event(e);
        if need_redraw {
            self.redraw(r)?;
        }
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
    /// The current action that the UI is taking.
    action: Action,
}

impl State {
    fn new(run: run::Run) -> Self {
        Self {
            cursor: 0,
            run,
            action: Action::default(),
        }
    }

    /// Gets whether the UI should be running.
    pub fn is_running(&self) -> bool {
        match self.action {
            Action::Quit => false,
            _ => true,
        }
    }

    /// Handles an event.  Returns true if the event changed the state.
    pub fn handle_event(&mut self, e: Event) -> bool {
        match e {
            Event::CursorDown => self.move_cursor_down(),
            Event::CursorUp => self.move_cursor_up(),
            Event::Quit => {
                self.action = Action::Quit;
                false
            }
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

/// High-level event, translated from a SDL event.
enum Event {
    /// Move the cursor up.
    CursorUp,
    /// Move the cursor down.
    CursorDown,
    /// Quit the program.
    Quit,
}

impl Event {
    fn from_sdl(e: sdl2::event::Event) -> Option<Self> {
        use sdl2::event::Event;
        match e {
            Event::Quit { .. } => Some(Self::Quit),
            Event::KeyDown {
                keycode: Some(k), ..
            } => Self::from_key(k),
            _ => None,
        }
    }

    fn from_key(k: sdl2::keyboard::Keycode) -> Option<Self> {
        use sdl2::keyboard::Keycode;
        match k {
            Keycode::J | Keycode::Down => Some(Self::CursorDown),
            Keycode::K | Keycode::Up => Some(Self::CursorUp),
            sdl2::keyboard::Keycode::Escape => Some(Self::Quit),
            _ => None,
        }
    }
}

enum Action {
    /// Run is inactive.
    Inactive,
    /// Currently navigating the splits.
    //Nav,
    /// Currently entering a field in the active split.
    //Entering{ field: time::Field, entry: String },
    /// ZombieSplit is quitting.
    Quit,
}

/// The default [Action] is [Action::Inactive].
impl Default for Action {
    fn default() -> Self {
        Action::Inactive
    }
}
