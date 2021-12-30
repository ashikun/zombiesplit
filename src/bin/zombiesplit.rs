use clap::{crate_authors, crate_version, App, Arg};
use std::sync::Arc;
use zombiesplit::model::attempt::observer::Observable;
use zombiesplit::{cli, config, ui, Manager};

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { cli::handle_error(run().await) });
}

async fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg_path = matches.value_of("config").unwrap().to_string();

    let cfg_raw = std::fs::read_to_string(&cfg_path)?;
    let cfg = config::System::load(&cfg_raw)?;
    let mut manager = Manager::new(cfg)?;

    let (pobs, ppump) = ui::presenter::observer();
    let pobs: std::sync::Arc<dyn zombiesplit::model::attempt::Observer> = std::sync::Arc::new(pobs);
    manager.add_observer(std::sync::Arc::downgrade(&pobs));

    let mut server = manager.server(&cli::get_short_descriptor(&matches)?)?;
    let mut afwd = server.handler();

    let ui = tokio::spawn(async move {
        // Yes, we're having to read the configuration in twice.  This is horrible, but it's a
        // temporary thing.
        let cfg_raw = std::fs::read_to_string(cfg_path)?;
        let cfg = config::System::load(&cfg_raw)?;

        let sdl = ui::sdl::Manager::new(&cfg.ui)?;
        let mut ui = ui::Instance::new(&cfg.ui, &sdl, &mut afwd, ppump)?;
        ui.run()?;
        anyhow::Result::<()>::Ok(())
    });

    server.run().await?;
    ui.await??;

    Ok(())
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("zombiesplit")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .help("use this system config file")
                .long("config")
                .default_value("sys.toml"),
        )
        .arg(Arg::with_name("game").help("The game to run").index(1))
        .arg(
            Arg::with_name("category")
                .help("The category to run")
                .index(2),
        )
}
