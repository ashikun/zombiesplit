use sdl2::{image::LoadTexture, rect::Point};
use zombiesplit::config::game;
use thiserror::Error;

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    let cfg = game::Game::load("soniccd.toml").expect("couldn't open config file");
    zombiesplit::ui::run(&cfg)?;

    Ok(())
}

