use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use zombiesplit::{cli, config, model::history::timing::Level, zombie, Db};

fn main() {
    cli::handle_error(run());
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg_raw = std::fs::read_to_string(matches.value_of("config").unwrap())?;
    let cfg = config::System::load(&cfg_raw)?;
    let mut db = Db::new(cfg.db_path)?;

    match matches.subcommand() {
        Some(("init", _)) => Ok(db.init()?),
        Some(("add-game", sub_m)) => run_add_game(&mut db, sub_m),
        Some(("add-run", sub_m)) => run_add_run(&mut db, sub_m),
        Some(("list-categories", sub_m)) => run_list_categories(&db, sub_m),
        Some(("list-runs", sub_m)) => run_list_runs(&db, sub_m),
        Some(("split-pbs", sub_m)) => run_split_pbs(&db, sub_m),
        Some(("pb", sub_m)) => run_pb(&db, sub_m),
        _ => Ok(()),
    }
}

fn run_add_game(db: &mut Db, matches: &ArgMatches) -> anyhow::Result<()> {
    let path = matches.value_of("game").ok_or(cli::Error::Game)?;
    zombie::add_game(db, path)?;
    Ok(())
}

fn run_list_categories(db: &Db, _matches: &ArgMatches) -> anyhow::Result<()> {
    zombie::list_game_categories(db)?;
    Ok(())
}

fn run_list_runs(db: &Db, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie::list_runs(db, &cli::get_short_descriptor(matches)?)?;
    Ok(())
}

fn run_split_pbs(db: &Db, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie::split_pbs(db, &cli::get_short_descriptor(matches)?)?;
    Ok(())
}

fn run_pb(db: &Db, matches: &ArgMatches) -> anyhow::Result<()> {
    zombie::run_pb(
        db,
        &cli::get_short_descriptor(matches)?,
        timing_level(matches),
    )?;
    Ok(())
}

fn run_add_run(db: &mut Db, matches: &ArgMatches) -> anyhow::Result<()> {
    let path = matches.value_of("run").ok_or(cli::Error::Run)?;
    zombie::add_run(db, path)?;
    Ok(())
}

fn timing_level(matches: &ArgMatches) -> Level {
    // TODO(@MattWindsor91): fromstr this
    match matches.value_of("level") {
        Some("totals") => Level::Totals,
        Some("full") => Level::Full,
        None | Some(/* "summary" */ _) => Level::Summary,
    }
}

fn app<'a>() -> App<'a> {
    App::new("zsdb")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("config")
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

fn init_subcommand<'a>() -> App<'a> {
    App::new("init").about("Initialises zombiesplit's database")
}

fn list_categories_subcommand<'a>() -> App<'a> {
    App::new("list-categories").about("lists all game/category pairs")
}

fn list_runs_subcommand<'a>() -> App<'a> {
    App::new("list-runs")
        .about("lists all runs stored for a category")
        .arg(Arg::new("game").help("The game to query").index(1))
        .arg(Arg::new("category").help("The category to query").index(2))
}

fn split_pbs_subcommand<'a>() -> App<'a> {
    App::new("split-pbs")
        .about("lists all split PBs for a category")
        .arg(Arg::new("game").help("The game to query").index(1))
        .arg(Arg::new("category").help("The category to query").index(2))
}

fn pb_subcommand<'a>() -> App<'a> {
    App::new("pb")
        .about("gets the PB for a category")
        .arg(
            Arg::new("level")
                .help("The level of timing information to get.")
                .long("level")
                .takes_value(true)
                .possible_values(&["summary", "totals", "full"]),
        )
        .arg(Arg::new("game").help("The game to query").index(1))
        .arg(Arg::new("category").help("The category to query").index(2))
}

fn run_subcommand<'a>() -> App<'a> {
    App::new("run")
        .about("starts a zombiesplit session")
        .arg(Arg::new("game").help("The game to run").index(1))
        .arg(Arg::new("category").help("The category to run").index(2))
}

fn add_game_subcommand<'a>() -> App<'a> {
    App::new("add-game")
        .about("adds a game from its TOML description")
        .arg(Arg::new("game").help("Path to game file to load").index(1))
}

fn add_run_subcommand<'a>() -> App<'a> {
    App::new("add-run")
        .about("adds a run from its TOML description")
        .arg(Arg::new("run").help("Path to run file to load").index(1))
}

#[cfg(test)]
mod test {
    use super::*;

    /// Checks that the clap app works properly.
    #[test]
    fn verify_app() {
        app().debug_assert();
    }
}
