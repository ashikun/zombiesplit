//! SQL for initialising the database.

pub(super) const GAME_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    game
        ( id    INTEGER PRIMARY KEY
        , short TEXT UNIQUE
        , name  TEXT 
        );
";

pub(super) const CATEGORY_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    category
        ( id     INTEGER PRIMARY KEY
        , short  TEXT
        , name   TEXT
        );
";

pub(super) const GAME_CATEGORY_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    game_category
        ( id          INTEGER PRIMARY KEY
        , gameid      INTEGER NOT NULL
        , categoryid  INTEGER NOT NULL
        , FOREIGN KEY(gameid)     REFERENCES game(id)
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        , UNIQUE(gameid, categoryid)
        );
";

pub(super) const SEGMENT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    segment
        ( id     INTEGER PRIMARY KEY
        , short  TEXT
        , name   TEXT
        );
";

pub(super) const CATEGORY_SEGMENT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    category_segment
        ( id          INTEGER PRIMARY KEY
        , categoryid  INTEGER NOT NULL
        , segmentid   INTEGER NOT NULL
        , position    INTEGER
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        , FOREIGN KEY(segmentid)  REFERENCES segment(id)
        , UNIQUE(categoryid, segmentid)
        , UNIQUE(categoryid, position)
        );
";

pub(super) const SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    split
        ( id    INTEGER PRIMARY KEY
        , name  TEXT
        );
";

pub(super) const SEGMENT_SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    segment_split
        ( id         INTEGER PRIMARY KEY
        , segmentid  INTEGER NOT NULL
        , splitid    INTEGER NOT NULL
        , position   INTEGER
        , FOREIGN KEY(segmentid) REFERENCES segment(id)
        , FOREIGN KEY(splitid)   REFERENCES split(id)
        , UNIQUE(segmentid, splitid)
        , UNIQUE(segmentid, position)
        );
";

pub(super) const RUN_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    run
        ( id          INTEGER PRIMARY KEY
        , timestamp   INTEGER
        , categoryid  INTEGER
        , FOREIGN KEY(categoryid) REFERENCES category(id)
        );
";

pub(super) const RUN_SPLIT_SQL: &str = "
CREATE TABLE IF NOT EXISTS
    run_split
        ( id       INTEGER PRIMARY KEY
        , runid    INTEGER
        , splitid  INTEGER
        , time     INTEGER
        , FOREIGN KEY(runid)   REFERENCES run(id)
        , FOREIGN KEY(splitid) REFERENCES split(id)
        , UNIQUE(runid, splitid)
        );
";
