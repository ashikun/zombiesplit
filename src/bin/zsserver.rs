//! The zombiesplit server binary.

use clap::{crate_authors, crate_version, App, Arg};
use zombiesplit::{cli, config, net};

#[tokio::main]
async fn main() {
    cli::handle_error(run().await)
}

async fn run() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let matches = app().get_matches();

    let cfg_path = matches.value_of("config").unwrap().to_string();
    let cfg_raw = std::fs::read_to_string(&cfg_path)?;
    let cfg = config::System::load(&cfg_raw)?;

    let manager = net::server::Manager::new(cfg)?;
    let server = manager.server(&cli::get_short_descriptor(&matches)?)?;

    server.run().await;
    Ok(())
}

fn app() -> App<'static> {
    App::new("zsserver")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::new("config")
                .help("use this system config file")
                .long("config")
                .default_value("sys.toml"),
        )
        .arg(Arg::new("game").help("The game to run").index(1))
        .arg(Arg::new("category").help("The category to run").index(2))
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
