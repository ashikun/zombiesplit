use std::{fs::File, io::Read, path::Path};

use zombiesplit::model;

fn read_config(path: &Path) -> anyhow::Result<model::Config> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents)?)
}

fn main() {
    let cfg = read_config(Path::new("soniccd.toml")).expect("couldn't open config file");
    for (_, c) in cfg.categories {
        println!("{}", c.name);
    }
}
