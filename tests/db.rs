//! Tests the database functionality on an in-memory database.

use zombiesplit::{
    model::{
        game::{self, category::ShortDescriptor},
        history::{self, FullTiming},
        Loadable,
    },
    Db,
};

const SAMPLE_GAME_PATH: &str = "scd11.toml";
const SAMPLE_GAME_NAME: &str = "scd11";
const SAMPLE_CATEGORY_NAME: &str = "btg-sonic";

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

fn short_descriptor() -> ShortDescriptor {
    ShortDescriptor::new(SAMPLE_GAME_NAME, SAMPLE_CATEGORY_NAME)
}

/// Tests initialising the database and getting a session out of it.
#[test]
fn test_sample_session() {
    let game = load_game();
    let db = setup_db(&game);

    let session = db
        .init_session(&short_descriptor())
        .expect("couldn't init session");
    assert_eq!(game.name, session.metadata.game);
    assert_eq!(
        game.categories
            .get(SAMPLE_CATEGORY_NAME)
            .map(|x| x.name.to_owned()),
        Some(session.metadata.category)
    );
}

/// Tests initialising the database and adding a run to it.
#[test]
fn test_sample_add_run() {
    let game = load_game();
    let mut db = setup_db(&game);

    let run = history::Run::<ShortDescriptor, FullTiming>::from_toml_file("scd11-pb.toml")
        .expect("couldn't load run");

    db.add_run(&run).expect("couldn't insert run");

    let runs = db
        .runs_for(&short_descriptor())
        .expect("couldn't get run summaries");
    assert_eq!(1, runs.len(), "there should only be one run")

    // TODO()
}
