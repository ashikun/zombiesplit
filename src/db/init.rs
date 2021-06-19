//! SQL for initialising the database.

pub(super) const SCHEMA: &str = "
BEGIN;

CREATE TABLE
    game
        ( game_id  INTEGER PRIMARY KEY
        , short    TEXT UNIQUE
        , name     TEXT 
        );
CREATE TABLE
    category
        ( category_id  INTEGER PRIMARY KEY
        , short        TEXT
        , name         TEXT
        );
CREATE TABLE
    game_category
        ( game_category_id  INTEGER PRIMARY KEY
        , game_id           INTEGER NOT NULL REFERENCES game
        , category_id       INTEGER NOT NULL REFERENCES category
        , UNIQUE(game_id, category_id)
        );
CREATE TABLE
    segment
        ( segment_id  INTEGER PRIMARY KEY
        , short       TEXT
        , name        TEXT
        );
CREATE TABLE
    category_segment
        ( category_segment_id  INTEGER PRIMARY KEY
        , category_id          INTEGER NOT NULL REFERENCES category
        , segment_id           INTEGER NOT NULL REFERENCES segment
        , position             INTEGER
        , UNIQUE(category_id, segment_id)
        , UNIQUE(category_id, position)
        );
CREATE TABLE
    split
        ( split_id  INTEGER PRIMARY KEY
        , short     TEXT
        , name      TEXT
        );
CREATE TABLE
    segment_split
        ( segment_split_id  INTEGER PRIMARY KEY
        , segment_id        INTEGER NOT NULL REFERENCES segment
        , split_id          INTEGER NOT NULL REFERENCES split
        , position          INTEGER
        , UNIQUE(segment_id, split_id)
        , UNIQUE(segment_id, position)
        );
CREATE TABLE
    run
        ( run_id            INTEGER PRIMARY KEY
        , is_completed      INTEGER  -- 0 = not completed, 1 = completed
        , timestamp         INTEGER  -- UNIX timestamp
        , game_category_id  INTEGER REFERENCES game_category
        );
CREATE TABLE
    run_split
        ( run_split_id  INTEGER PRIMARY KEY
        , run_id        INTEGER NOT NULL REFERENCES run
        , split_id      INTEGER NOT NULL REFERENCES split
        , time_ms       INTEGER  -- sum of all times logged for the split
        , UNIQUE(run_id, split_id)
        );

COMMIT;
";
