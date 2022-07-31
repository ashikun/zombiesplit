//! The zombiesplit client.

use clap::Parser;
use zombiesplit::{cli, config::Client as Config, net, ui};

fn main() {
    cli::handle_error(run())
}

/// Graphical client for zombiesplit.
#[derive(Parser, Debug)]
#[clap(name = "zsclient", about, version, author)]
struct Args {
    /// Use this system config file
    #[clap(short, long)]
    config: Option<std::path::PathBuf>,
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let args = Args::parse();
    let cfg = Config::load(args.config)?;

    let (pobs, ppump) = ui::presenter::observer::make();

    let mut client = net::client::Sync::new(cfg.server_addr, pobs)?;
    client.run(|state, client| {
        let vconf = cfg.ui.into_view_config()?;
        let sdl = ui::sdl::Manager::new(&vconf)?;
        let presenter = ui::presenter::Presenter::new(client, &state);
        let mut ui = ui::Instance::new(&vconf, &sdl, presenter, ppump)?;
        ui.run().map_err(anyhow::Error::from)
    })
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
