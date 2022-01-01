//! The zombiesplit client.

use clap::{crate_authors, crate_version, App, Arg};
use zombiesplit::{cli, config, net, ui};

fn main() {
    cli::handle_error(run())
}

fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();
    let cfg_path = matches.value_of("config").unwrap().to_string();

    // TODO(@MattWindsor91): separate client and server config?
    let cfg_raw = std::fs::read_to_string(cfg_path)?;
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

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("zsclient")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .help("use this system config file")
                .long("config")
                .default_value("sys.toml"),
        )
}
