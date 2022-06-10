//! Tests the database functionality on an in-memory database.

use std::{convert::TryFrom, ops::Add, rc::Rc};
use tempfile::{tempdir, TempDir};
use zombiesplit::{
    db::{self, Db, Reader, Sink},
    model::{
        game::{self, category::ShortDescriptor},
        history,
        session::{
            self,
            action::{Action, Handler, OldDestination},
            event,
        },
        short, Loadable, Time,
    },
};

const SAMPLE_GAME_PATH: &str = "scd11.toml";
const SAMPLE_RUN_PATH: &str = "scd11-pb.toml";
const SAMPLE_GAME_NAME: &str = "scd11";
const SAMPLE_CATEGORY_NAME: &str = "btg-sonic";

fn load_game() -> game::Config {
    game::Config::from_toml_file(SAMPLE_GAME_PATH).expect("couldn't load sample game")
}

fn setup_db(game: &game::Config, in_dir: &TempDir) -> Db {
    let mut file = in_dir.path().to_path_buf();
    file.push("test.db");

    let db = Db::new(file).expect("couldn't open db in memory");
    db.init().expect("couldn't initialise database");

    db.add_game(SAMPLE_GAME_NAME, game)
        .expect("couldn't add game to database");

    db
}

fn short_descriptor() -> ShortDescriptor {
    ShortDescriptor::new(SAMPLE_GAME_NAME, SAMPLE_CATEGORY_NAME)
}

fn init_session(handle: &Reader, snk: Sink) -> session::Session<event::observer::Null> {
    let mut insp = handle
        .inspect(&short_descriptor())
        .expect("couldn't open category db");
    let mut ses = insp
        .init_session(&event::observer::Null)
        .expect("couldn't init session");
    ses.set_sink(Box::new(snk));
    ses
}

/// Tests initialising the database and getting a session out of it.
#[test]
fn test_sample_session() {
    let tdir = tempdir().expect("can't open dir");

    let game = load_game();
    let db = Rc::new(setup_db(&game, &tdir));
    let handle = db.reader().expect("couldn't open reader");

    let mut session = init_session(&handle, Sink::new(db));
    let dump = session.dump().expect("session shouldn't fail to dump");
    assert_eq!(game.name, dump.attempt.category.game);
    assert_eq!(
        game.categories
            .get(&short::Name::from(SAMPLE_CATEGORY_NAME))
            .map(|x| x.name.to_owned()),
        Some(dump.attempt.category.category)
    );
}

/// Tests initialising the database and adding a run to it.
#[test]
fn test_sample_add_run() {
    let tdir = tempdir().expect("can't open dir");

    let game = load_game();
    let db = setup_db(&game, &tdir);

    let run = history::run::FullyTimed::<ShortDescriptor>::from_toml_file(SAMPLE_RUN_PATH)
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
    let tdir = tempdir().expect("can't open dir");

    let game = load_game();
    let db = Rc::new(setup_db(&game, &tdir));
    let handle = db.reader().expect("couldn't open reader");

    let mut session = init_session(&handle, Sink::new(db.clone()));

    // This shouldn't insert a run.
    session
        .handle(Action::NewRun(OldDestination::Save))
        .unwrap();

    let time = Time::try_from(8675309).expect("time didn't parse");

    // This also shouldn't.
    session.set_timestamper(chrono::Utc::now);
    inject_session_actions(
        &mut session,
        vec![Action::Push(0, time)],
        OldDestination::Discard,
    );

    // This should.
    session.set_timestamper(chrono::Utc::now);
    inject_session_actions(
        &mut session,
        vec![Action::Push(0, time)],
        OldDestination::Save,
    );

    // As should this.
    // (We change the timestamp to avoid having the database reject the run as
    // a duplicate.)
    session.set_timestamper(|| chrono::Utc::now().add(chrono::Duration::weeks(1)));
    inject_session_actions(
        &mut session,
        vec![Action::Push(0, time), Action::Push(1, time)],
        OldDestination::Save,
    );

    let rd = db.reader().expect("couldn't get reader");
    let mut insp = rd
        .inspect(&short_descriptor())
        .expect("couldn't get inspector");

    let runs = db
        .runs_for(&short_descriptor())
        .expect("couldn't get run summaries");
    assert_eq!(2, runs.len(), "there should be two runs");

    let run = &runs[0];
    assert_eq!(run.timing.total, time);
    let run = &runs[1];
    assert_eq!(run.timing.total, time + time);

    assert!(
        runs[0].date < runs[1].date,
        "the timestamper should have given these different times"
    );

    for (i, run) in runs.iter().enumerate() {
        let s_run = insp
            .run_at_index(i, &db::inspect::Summary)
            .expect("Unexpected error getting index")
            .expect("Unexpected omission getting index");
        assert_eq!(run, &s_run);
    }
}

fn inject_session_actions(
    session: &mut session::Session<event::observer::Null>,
    actions: Vec<Action>,
    dest: OldDestination,
) {
    for action in actions {
        session.handle(action).unwrap()
    }
    session.handle(Action::NewRun(dest)).unwrap();
}
