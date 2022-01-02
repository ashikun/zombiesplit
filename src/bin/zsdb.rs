use clap::{Parser, Subcommand};
use zombiesplit::model::game::category::ShortDescriptor;
use zombiesplit::{
    cli, config,
    model::{history::timing::Level, short},
    zombie, Db,
};

fn main() {
    cli::handle_error(run());
}

/// Database manager for zombiesplit.
#[derive(Parser, Debug)]
#[clap(name = "zsdb", about, version, author)]
struct Args {
    /// Use this system config file
    #[clap(short, long, default_value = "sys.toml")]
    config: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialises zombiesplit's database
    Init,
    /// Lists all game/category pairs
    Categories,
    // Lists all runs stored for a category
    Runs {
        /// The game to query
        game: short::Name,
        /// The category to query
        category: short::Name,
    },
    /// Lists all split PBs for a category
    SplitPBs {
        /// The game to query
        game: short::Name,
        /// The category to query
        category: short::Name,
    },
    /// Gets the PB for a category
    PB {
        /// The game to query
        game: short::Name,
        /// The category to query
        category: short::Name,
        /// The level of timing information to get
        #[clap(short, long, default_value_t)]
        level: Level,
    },
    /// Adds a game from its TOML description
    AddGame {
        /// Path to game file to load
        path: String,
    },
    /// Adds a run from its TOML description
    AddRun {
        /// Path to run file to load
        path: String,
    },
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let args = Args::parse();
    let cfg_raw = std::fs::read_to_string(args.config)?;
    let cfg = config::System::load(&cfg_raw)?;
    let mut db = Db::new(cfg.db_path)?;

    match args.command {
        Command::Init => db.init()?,
        Command::AddGame { path } => zombie::add_game(&mut db, path)?,
        Command::AddRun { path } => zombie::add_run(&mut db, path)?,
        Command::Categories => zombie::list_game_categories(&db)?,
        Command::Runs { game, category } => {
            zombie::list_runs(&db, &ShortDescriptor::new(game, category))?
        }
        Command::SplitPBs { game, category } => {
            zombie::split_pbs(&db, &ShortDescriptor::new(game, category))?
        }
        Command::PB {
            game,
            category,
            level,
        } => zombie::run_pb(&db, &ShortDescriptor::new(game, category), level)?,
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    /// Checks that the clap app works properly.
    #[test]
    fn verify_app() {
        use clap::IntoApp;
        Args::into_app().debug_assert();
    }
}
