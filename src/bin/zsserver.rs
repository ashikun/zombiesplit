//! The zombiesplit server binary.

use clap::Parser;
use zombiesplit::model::game::category::ShortDescriptor;
use zombiesplit::{cli, config, model::short, net};

#[tokio::main]
async fn main() {
    cli::handle_error(run().await)
}

/// Server for zombiesplit.
#[derive(Parser, Debug)]
#[clap(name = "zsserver", about, version, author)]
struct Args {
    /// The game to run (for example, "scd11")
    game: short::Name,
    /// The category to run (for example, "btg-sonic")
    category: short::Name,

    /// Use this system config file
    #[clap(short, long, default_value = "sys.toml")]
    config: String,
}

async fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let args = Args::parse();

    let cfg_raw = std::fs::read_to_string(args.config)?;
    let cfg = config::System::load(&cfg_raw)?;

    let manager = net::server::Manager::new(cfg)?;
    let server = manager.server(&ShortDescriptor::new(args.game, args.category))?;

    server.run().await;
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
