//! The zombiesplit client.

use clap::Parser;
use zombiesplit::{cli, config, net, ui};

fn main() {
    cli::handle_error(run())
}

/// Graphical client for zombiesplit.
#[derive(Parser, Debug)]
#[clap(name = "zsclient", about, version, author)]
struct Args {
    /// Use this system config file
    #[clap(short, long, default_value = "sys.toml")]
    config: String,
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let args = Args::parse();

    // TODO(@MattWindsor91): separate client and server config?
    let cfg_raw = std::fs::read_to_string(args.config)?;
    let cfg = config::System::load(&cfg_raw)?;

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
