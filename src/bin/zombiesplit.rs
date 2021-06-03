use zombiesplit::config;

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    let sys = config::System::load("sys.toml")?;
    let cfg = config::Game::load("soniccd.toml")?;
    let run = cfg.to_run("btg")?;
    let p = zombiesplit::Presenter::new(run);
    zombiesplit::View::new(sys.ui)?.spawn(p)?.run()?;

    Ok(())
}
