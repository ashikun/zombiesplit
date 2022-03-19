//! The zombiesplit client.

use clap::Parser;
use zombiesplit::{cli, config::Client as Config, model::session::action::Handler, net, ui};

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
    // TODO(@MattWindsor91): server info
    let state = client.dump()?;

    let (csend, crecv) = tokio::sync::oneshot::channel();

    let mut observing_client = client.clone();
    let _handle = std::thread::spawn(move || -> anyhow::Result<()> {
        observing_client.observe(crecv)?;
        Ok(())
    });

    let vconf = cfg.ui.into_view_config()?;
    let sdl = ui::sdl::Manager::new(&vconf)?;
    let presenter = ui::presenter::Presenter::new(&mut client, &state);
    let mut ui = ui::Instance::new(&vconf, &sdl, presenter, ppump)?;
    ui.run()?;

    csend
        .send(())
        .map_err(|_| anyhow::Error::msg("couldn't cancel observer"))?;

    // TODO(@MattWindsor91): make this work

    /*
    handle
        .join()
        .map_err(|e| anyhow::anyhow!("couldn't join client thread"))??;
     */

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
