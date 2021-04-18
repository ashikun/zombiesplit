use zombiesplit::config::game;

fn main() {
    let cfg = game::Game::load("soniccd.toml").expect("couldn't open config file");
    for (_, c) in cfg.categories {
        println!("{}", c.name);
    }
}
