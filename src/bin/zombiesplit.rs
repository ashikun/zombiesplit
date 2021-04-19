use zombiesplit::config::game;
use thiserror::Error;

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    let sdl_ctx = sdl2::init().map_err(Error::SdlInit)?;
    let video = sdl_ctx.video().map_err(Error::SdlInit)?;

    let window = video.window("zombiesplit", 320, 640)
        .position_centered()
        .build()
        .map_err(Error::SdlWindow)?;

    let cfg = game::Game::load("soniccd.toml").expect("couldn't open config file");
    for (_, c) in cfg.categories {
        println!("{}", c.name);
    }

    let mut canvas = window.into_canvas().build().map_err(Error::SdlInteger)?;

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

#[derive(Debug, Error)]
enum Error {
    /// An error occurred while initialising an SDL subsystem.
    #[error("SDL init error: {0}")]
    SdlInit(String),

    /// An error occurred while building a window.
    #[error("SDL windowing error")]
    SdlWindow(#[from] sdl2::video::WindowBuildError),

    /// An error occurred while building a window.
    #[error("SDL error")]
    SdlInteger(#[from] sdl2::IntegerOrSdlError)
}
