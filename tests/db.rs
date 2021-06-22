//! Tests the database functionality on an in-memory database.

use zombiesplit::{Db, model::{Loadable, history, game::{self, category::ShortDescriptor}}};

const SAMPLE_GAME_PATH: &str = "soniccd.toml";
const SAMPLE_GAME_NAME: &str = "soniccd";
const SAMPLE_CATEGORY_NAME: &str = "btgs";

fn load_game() -> game::Config {
    game::Config::from_toml_file(SAMPLE_GAME_PATH).expect("couldn't load sample game")
}

fn setup_db(game: &game::Config) -> Db {
    let mut db = Db::in_memory().expect("couldn't open db in memory");
    db.init().expect("couldn't initialise database");

    db.add_game(SAMPLE_GAME_NAME, game)
        .expect("couldn't add game to database");
    
    db
}

/// Tests initialising the database and getting a session out of it.
#[test]
fn test_sample_session() {
    let game = load_game();
    let db = setup_db(&game);

    let session = db
        .init_session(&ShortDescriptor::new(SAMPLE_GAME_NAME, SAMPLE_CATEGORY_NAME))
        .expect("couldn't init session");
    assert_eq!(game.name, session.metadata.game);
    assert_eq!(
        game.categories.get("btgs").map(|x| x.name.to_owned()),
        Some(session.metadata.category)
    );
}

/// Tests initialising the database and adding a run to it.
#[test]
fn test_sample_add_run() {
    let game = load_game();
    let mut db = setup_db(&game);

    let run = history::Run::<ShortDescriptor>::from_toml_file("soniccd-pb.toml").expect("couldn't load run");

    db.add_run(&run).expect("couldn't insert run")

    // TODO()
}
