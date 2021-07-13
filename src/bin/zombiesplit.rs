use clap::{crate_authors, crate_version, App, Arg, ArgMatches, SubCommand};
use thiserror::Error;
use zombiesplit::{
    config,
    model::{game::category::ShortDescriptor, history::timing::Level},
    Zombie,
};

fn main() {
    run().unwrap()
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg = config::System::load(matches.value_of("config").unwrap())?;
    let zombie = Zombie::new(cfg)?;

    match matches.subcommand() {
        ("init", Some(sub_m)) => run_init(zombie, sub_m),
        ("add-game", Some(sub_m)) => run_add_game(zombie, sub_m),
        ("add-run", Some(sub_m)) => run_add_run(zombie, sub_m),
        ("list-categories", Some(sub_m)) => run_list_categories(zombie, sub_m),
        ("list-runs", Some(sub_m)) => run_list_runs(zombie, sub_m),
        ("split-pbs", Some(sub_m)) => run_split_pbs(zombie, sub_m),
        ("pb", Some(sub_m)) => run_pb(zombie, sub_m),
        ("run", Some(sub_m)) => run_run(zombie, sub_m),
        _ => Ok(()),
    }
}

fn run_init(zombie: Zombie, _matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.init_db()?;
    Ok(())
}

fn run_add_game(mut zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    let path = matches.value_of("game").ok_or(Error::Game)?;
    zombie.add_game(path)?;
    Ok(())
}

fn run_list_categories(zombie: Zombie, _matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.list_game_categories()?;
    Ok(())
}

fn run_list_runs(zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.list_runs(&get_short_descriptor(matches)?)?;
    Ok(())
}

fn run_split_pbs(zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.split_pbs(&get_short_descriptor(matches)?)?;
    Ok(())
}

fn run_pb(zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.run_pb(&get_short_descriptor(matches)?, timing_level(matches))?;
    Ok(())
}

fn run_add_run(mut zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    let path = matches.value_of("run").ok_or(Error::Run)?;
    zombie.add_run(path)?;
    Ok(())
}

fn run_run(zombie: Zombie, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie.run(&get_short_descriptor(matches)?)?;
    Ok(())
}

fn get_short_descriptor(matches: &ArgMatches) -> Result<ShortDescriptor, Error> {
    let game = matches.value_of("game").ok_or(Error::Game)?;
    let category = matches.value_of("category").ok_or(Error::Category)?;
    Ok(ShortDescriptor::new(game, category))
}

fn timing_level(matches: &ArgMatches) -> Level {
    // TODO(@MattWindsor91): fromstr this
    match matches.value_of("level") {
        Some("totals") => Level::Totals,
        Some("full") => Level::Full,
        None | Some(/* "summary" */ _) => Level::Summary,
    }
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
        .subcommand(init_subcommand())
        .subcommand(add_game_subcommand())
        .subcommand(add_run_subcommand())
        .subcommand(list_categories_subcommand())
        .subcommand(list_runs_subcommand())
        .subcommand(split_pbs_subcommand())
        .subcommand(run_subcommand())
        .subcommand(pb_subcommand())
}

fn init_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init").about("Initialises zombiesplit's database")
}

fn list_categories_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list-categories").about("lists all game/category pairs")
}

fn list_runs_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list-runs")
        .about("lists all runs stored for a category")
        .arg(Arg::with_name("game").help("The game to query").index(1))
        .arg(
            Arg::with_name("category")
                .help("The category to query")
                .index(2),
        )
}

fn split_pbs_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("split-pbs")
        .about("lists all split PBs for a category")
        .arg(Arg::with_name("game").help("The game to query").index(1))
        .arg(
            Arg::with_name("category")
                .help("The category to query")
                .index(2),
        )
}

fn pb_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("pb")
        .about("gets the PB for a category")
        .arg(
            Arg::with_name("level")
                .help("The level of timing information to get.")
                .long("level")
                .takes_value(true)
                .possible_values(&["summary", "totals", "full"]),
        )
        .arg(Arg::with_name("game").help("The game to query").index(1))
        .arg(
            Arg::with_name("category")
                .help("The category to query")
                .index(2),
        )
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

fn add_game_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add-game")
        .about("adds a game from its TOML description")
        .arg(
            Arg::with_name("game")
                .help("Path to game file to load")
                .index(1),
        )
}

fn add_run_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add-run")
        .about("adds a run from its TOML description")
        .arg(
            Arg::with_name("run")
                .help("Path to run file to load")
                .index(1),
        )
}

#[derive(Debug, Error)]
enum Error {
    /// Error getting a category from the command line.
    #[error("no category provided")]
    Category,
    /// Error getting a game from the command line.
    #[error("no game provided")]
    Game,
    /// Error getting a run from the command line.
    #[error("no run provided")]
    Run,
}
