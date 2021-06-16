//! Tests the database functionality on an in-memory database.

use zombiesplit::{model::game, Db};

/// Tests initialising the database and inserting the sample game.
#[test]
fn test_sample_session() {
    let mut db = Db::in_memory().expect("couldn't open db in memory");
    db.init().expect("couldn't initialise database");

    let game = game::Config::load("soniccd.toml").expect("couldn't load sample game");
    db.add_game("soniccd", &game)
        .expect("couldn't add game to database");

    let session = db
        .init_session("soniccd", "btgs")
        .expect("couldn't init session");
    assert_eq!(game.name, session.metadata.game);
    assert_eq!(
        game.categories.get("btgs").map(|x| x.name.to_owned()),
        Some(session.metadata.category)
    );

    // TODO()
}
