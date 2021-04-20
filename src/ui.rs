//! The zombiesplit user interface.
pub mod error;
pub mod gfx;

pub use error::{Error, Result};
use sdl2::{image::LoadTexture, rect::Point};
use crate::config::game;

/// Runs the UI loop.
pub fn run(cfg: &game::Game) -> error::Result<()> {
    // TODO(@MattWindsor91): pass in something other than Game.
    let sdl_ctx = sdl2::init().map_err(Error::SdlInit)?;
    let video = sdl_ctx.video().map_err(Error::SdlInit)?;

    let window = gfx::make_window(&video)?;


    let mut screen = window.into_canvas().build().map_err(Error::SdlInteger)?;
    let textures = screen.texture_creator();

    let font = textures.load_texture("font.png").map_err(Error::SdlLoadFont)?;

    screen.set_draw_color(sdl2::pixels::Color::BLACK);
    screen.clear();

    let mut r = gfx::Renderer {
        screen,
        font
    };

    let mut tl = Point::new(10, 10);
    for (_, c) in cfg.categories.iter() {
        r.put_str(&c.name, tl)?;
        tl = gfx::metrics::offset(tl, 0, 1);
    }
    r.screen.present();

    let mut events = sdl_ctx.event_pump().map_err(Error::SdlInit)?;

    'running: loop {
        for event in events.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => { break 'running },
                _ => {}
            }
        }
    }

    Ok(())
}
