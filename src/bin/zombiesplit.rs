use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};
use thiserror::Error;
use zombiesplit::{config, Zombie};

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg = config::System::load(matches.value_of("config").unwrap())?;
    let zombie = Zombie::new(cfg)?;

    match matches.subcommand() {
        ("add", Some(sub_m)) => run_add(zombie, sub_m),
        ("init", Some(sub_m)) => run_init(zombie, sub_m),
        ("run", Some(sub_m)) => run_run(zombie, sub_m),
        _ => Ok(()),
    }
}

fn run_add(mut zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    let path = matches.value_of("game").ok_or(Error::NoGameProvided)?;
    zombie.add_game(path)?;
    Ok(())
}

fn run_init(zombie: Zombie, _matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.init_db()?;
    Ok(())
}

fn run_run(zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    let game = matches.value_of("game").ok_or(Error::NoGameProvided)?;
    let category = matches
        .value_of("category")
        .ok_or(Error::NoCategoryProvided)?;
    zombie.run(game, category)?;
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
        .subcommand(add_subcommand())
        .subcommand(init_subcommand())
        .subcommand(run_subcommand())
}

fn init_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init").about("Initialises zombiesplit's database")
}

fn run_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run")
        .about("starts a zombiesplit session")
        .arg(Arg::with_name("game").help("The game to run").index(1))
        .arg(
            Arg::with_name("category")
                .help("The category to run")
                .index(2),
        )
}

fn add_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("adds a game from its TOML description")
        .arg(
            Arg::with_name("game")
                .help("The game ID to add (TOML filename less .toml)")
                .index(1),
        )
}

#[derive(Debug, Error)]
enum Error {
    #[error("no game provided")]
    NoGameProvided,
    #[error("no category provided")]
    NoCategoryProvided,
}
