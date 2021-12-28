use clap::{crate_authors, crate_version, App, Arg};
use zombiesplit::{cli, config, ui, Manager};

fn main() {
    cli::handle_error(run());
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg_raw = std::fs::read_to_string(matches.value_of("config").unwrap())?;
    let cfg = config::System::load(&cfg_raw)?;
    let ui_cfg = cfg.ui.clone();
    let manager = Manager::new(cfg)?;
    let mut server = manager.run(&cli::get_short_descriptor(&matches)?)?;
    ui::run(ui_cfg, &mut server.session)?;
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
