//! The zombiesplit server binary.

use clap::Parser;
use zombiesplit::model::game::category::ShortDescriptor;
use zombiesplit::{cli, config::Server as Config, net};

#[tokio::main]
async fn main() {
    cli::handle_error(run().await)
}

/// Server for zombiesplit.
#[derive(Parser, Debug)]
#[clap(name = "zsserver", about, version, author)]
struct Args {
    /// The game/category to run (for example, "scd11/btg-sonic")
    target: ShortDescriptor,

    /// Use this system config file
    #[clap(short, long)]
    config: Option<std::path::PathBuf>,
}

async fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let args = Args::parse();
    let cfg = Config::load(args.config)?;

    let manager = net::server::Manager::new(cfg)?;
    let server = manager.server(&args.target)?;

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
        Args::command().debug_assert();
    }
}
