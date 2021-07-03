//! Tests the database functionality on an in-memory database.

use std::{convert::TryFrom, ops::Add, rc::Rc};
use zombiesplit::{
    db::{Db, Observer},
    model::{
        attempt::split::Set,
        game::{self, category::ShortDescriptor},
        history::{self, FullTiming},
        Loadable, Time,
    },
};

const SAMPLE_GAME_PATH: &str = "scd11.toml";
const SAMPLE_RUN_PATH: &str = "scd11-pb.toml";
const SAMPLE_GAME_NAME: &str = "scd11";
const SAMPLE_CATEGORY_NAME: &str = "btg-sonic";

fn load_game() -> game::Config {
    game::Config::from_toml_file(SAMPLE_GAME_PATH).expect("couldn't load sample game")
}

fn setup_db(game: &game::Config) -> Db {
    let db = Db::in_memory().expect("couldn't open db in memory");
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
    let db = setup_db(&game);

    let run = history::Run::<ShortDescriptor, FullTiming>::from_toml_file(SAMPLE_RUN_PATH)
        .expect("couldn't load run");

    db.add_run(&run).expect("couldn't insert run");

    let runs = db
        .runs_for(&short_descriptor())
        .expect("couldn't get run summaries");
    assert_eq!(1, runs.len(), "there should only be one run")

    // TODO(MattWindsor91): check run specifics
}

/// Tests initialising the database and adding a run through observation.
#[test]
fn test_sample_observe_run() {
    let game = load_game();
    let db = Rc::new(setup_db(&game));

    let mut session = db
        .init_session(&short_descriptor())
        .expect("couldn't init session");

    session.add_observer(Box::new(Observer::new(db.clone())));

    // This shouldn't insert a run.
    session.reset();

    let time = Time::try_from(8675309).expect("time didn't parse");

    // This should.
    session.set_timestamper(|| chrono::Utc::now());
    session.push_to(0, time);
    session.reset();

    // As should this.
    // (We change the timestamp to avoid having the database reject the run as
    // a duplicate.)
    session.set_timestamper(|| chrono::Utc::now().add(chrono::Duration::weeks(1)));
    session.push_to(0, time);
    session.push_to(1, time);
    session.reset();

    let runs = db
        .runs_for(&short_descriptor())
        .expect("couldn't get run summaries");
    assert_eq!(2, runs.len(), "there should be two runs");

    let run = &runs[0];
    assert_eq!(run.timing.total, time);
    let run = &runs[1];
    assert_eq!(run.timing.total, time + time);

    // TODO(MattWindsor91): check run specifics
}
