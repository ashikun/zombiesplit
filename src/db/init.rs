//! SQL for initialising the database.

use super::error::Result;
use std::ops::Deref;

/// Initialises the database at `conn`.
///
/// # Errors
///
/// Propagates any errors from the underlying SQL database.
pub fn on_db<C: Deref<Target = rusqlite::Connection>>(conn: C) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

/// The zombiesplit database schema.
const SCHEMA: &str = "
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
        , UNIQUE(game_id, category_id)  -- each game can have each category at most once
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
        , position             INTEGER NOT NULL  -- orders categories chronologically
        , UNIQUE(category_id, segment_id)        -- each category can have each segment at most once
        , UNIQUE(category_id, position)          -- each category can have multiple segments, but they must be totally ordered
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
        , position          INTEGER NOT NULL  -- orders splits chronologically
        , UNIQUE(segment_id, split_id)        -- each segment can have each split at most once
        , UNIQUE(segment_id, position)        -- each segment can have multiple splits, but they must be totally ordered
        );

-- Logs overall metadata for a run.
CREATE TABLE
    run
        ( run_id            INTEGER PRIMARY KEY
        , game_category_id  INTEGER REFERENCES game_category
        , timestamp         INTEGER                                              -- UNIX timestamp
        , is_completed      INTEGER CHECK(is_completed = 0 OR is_completed = 1)  -- 0 = not completed, 1 = completed
        , UNIQUE(game_category_id, timestamp)                                    -- these two being the same implies a duplicate run insertion
        );

-- Logs a set of times for a split in a particular run.
CREATE TABLE
    run_split
        ( run_split_id  INTEGER PRIMARY KEY
        , run_id        INTEGER NOT NULL REFERENCES run
        , split_id      INTEGER NOT NULL REFERENCES split
        , UNIQUE(run_id, split_id)                         -- each split can appear in a run at most once
        );

-- Logs a time for a split in a particular run.
CREATE TABLE
    run_split_time
        ( run_split_time_id INTEGER PRIMARY KEY
        , run_split_id      INTEGER NOT NULL REFERENCES run_split_id
        , position          INTEGER NOT NULL  -- orders split times chronologically
        , time_ms           INTEGER NOT NULL  -- the time itself, in milliseconds
        , UNIQUE(run_split_id, position)      -- each split can have multiple times entered for the run, but they must be totally ordered
        );

COMMIT;
";
