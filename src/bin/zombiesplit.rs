use zombiesplit::config::game;

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    let cfg = game::Game::load("soniccd.toml")?;
    let run = cfg.to_run("btg")?;
    zombiesplit::ui::Manager::new()?.spawn(run)?.run()?;

    Ok(())
}
