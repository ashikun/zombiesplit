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

    let (mut asend, arecv) = net::client::action::channel();
    let (pobs, ppump) = ui::presenter::observer();

    let _handle = std::thread::spawn(move || -> anyhow::Result<()> {
        net::client::Sync::new(cfg.server_addr, pobs, arecv)?.run()?;
        Ok(())
    });

    let sdl = ui::sdl::Manager::new(&cfg.ui)?;
    let mut ui = ui::Instance::new(&cfg.ui, &sdl, &mut asend, ppump)?;
    ui.run()?;

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