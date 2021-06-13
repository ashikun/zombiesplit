//! Tests the database functionality on an in-memory database.

use zombiesplit::{model::Game, Db};

/// Tests initialising the database and inserting the sample game.
#[test]
fn test_sample_session() {
    let db = Db::in_memory().expect("couldn't open db in memory");
    db.init().expect("couldn't initialise database");

    let game = Game::load("soniccd.toml").expect("couldn't load sample game");
    db.add_game("soniccd", &game)
        .expect("couldn't add game to database");

    let session = db
        .init_session("soniccd", "btg")
        .expect("couldn't init session");
    assert_eq!(game.name, session.metadata.game);
    assert_eq!(
        game.categories.get("btg").map(|x| x.name.to_owned()),
        Some(session.metadata.category)
    );

    // TODO()
}
