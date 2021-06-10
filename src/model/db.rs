//! Top-level module for the model's sqlite database.

pub mod error;
use std::path::Path;

pub use error::{Error, Result};

/// A connection to zombiesplit's database.
pub struct Db {
    conn: rusqlite::Connection,
}

impl Db {
    /// Opens a database connection to a given file.
    ///
    /// # Errors
    ///
    /// Returns errors from the underlying database library if the connection
    /// opening failed.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            conn: rusqlite::Connection::open(path)?,
        })
    }

    /// Initialises the database for first use.
    ///
    /// # Errors
    ///
    /// Propagates errors from the database if anything goes wrong.
    pub fn init(&self) -> Result<()> {
        for (name, ddl) in &[
            ("game", INIT_GAME_SQL),
            ("category", INIT_CATEGORY_SQL),
            ("game_category", INIT_GAME_CATEGORY_SQL),
            ("segment", INIT_SEGMENT_SQL),
            ("category_segment", INIT_CATEGORY_SEGMENT_SQL),
            ("split", INIT_SPLIT_SQL),
            ("segment_split", INIT_SEGMENT_SPLIT_SQL),
            ("run", INIT_RUN_SQL),
            ("run_split", INIT_RUN_SPLIT_SQL),
        ] {
            eprintln!("creating table {}", name);
            let _ = self.conn.execute(ddl, [])?;
        }
        Ok(())
    }
}

const INIT_GAME_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    game
        ( id    TEXT PRIMARY KEY
        , name  TEXT 
        );
";

const INIT_CATEGORY_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    category
        ( id     INTEGER PRIMARY KEY
        , short  TEXT
        , name   TEXT
        );
";

const INIT_GAME_CATEGORY_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    game_category
        ( id          INTEGER PRIMARY KEY
        , gameid      INTEGER NOT NULL
        , categoryid  INTEGER NOT NULL
        , FOREIGN KEY(gameid)     REFERENCES game(id)
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        );
";

const INIT_SEGMENT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    segment
        ( id     INTEGER PRIMARY KEY
        , short  TEXT
        , name   TEXT
        );
";

const INIT_CATEGORY_SEGMENT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    category_segment
        ( id          INTEGER PRIMARY KEY
        , categoryid  INTEGER NOT NULL
        , segmentid   INTEGER NOT NULL
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        , FOREIGN KEY(segmentid)  REFERENCES segment(id)
        );
";

const INIT_SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    split
        ( id    INTEGER PRIMARY KEY
        , name  TEXT
        );
";

const INIT_SEGMENT_SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    segment_split
        ( id         INTEGER PRIMARY KEY
        , segmentid  INTEGER NOT NULL
        , splitid    INTEGER NOT NULL
        , position   INTEGER
        , FOREIGN KEY(segmentid) REFERENCES segment(id)
        , FOREIGN KEY(splitid)   REFERENCES split(id)
        );
";

const INIT_RUN_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    run
        ( id          INTEGER PRIMARY KEY
        , timestamp   INTEGER
        , categoryid  INTEGER
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        );
";

const INIT_RUN_SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    run_split
        ( id       INTEGER PRIMARY KEY
        , runid    INTEGER
        , splitid  INTEGER
        , time     INTEGER
        , FOREIGN KEY(runid)   REFERENCES run(id)
        , FOREIGN KEY(splitid) REFERENCES split(id)
        );
";
