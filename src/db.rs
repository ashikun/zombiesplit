//! Top-level module for the model's sqlite database.

pub mod error;
mod init;
use crate::model::{game, split::Split, Metadata, Run, Session};
use rusqlite::params;
use std::{collections::HashMap, path::Path};

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
            ("game", init::GAME_SQL),
            ("category", init::CATEGORY_SQL),
            ("game_category", init::GAME_CATEGORY_SQL),
            ("segment", init::SEGMENT_SQL),
            ("category_segment", init::CATEGORY_SEGMENT_SQL),
            ("split", init::SPLIT_SQL),
            ("segment_split", init::SEGMENT_SPLIT_SQL),
            ("run", init::RUN_SQL),
            ("run_split", init::RUN_SPLIT_SQL),
        ] {
            log::info!("creating table {}", name);
            let _ = self.conn.execute(ddl, [])?;
        }
        Ok(())
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&self, short: &str, game: &game::Game) -> Result<()> {
        log::info!("adding game {}", short);

        self.conn.execute(
            "INSERT INTO game (short, name) VALUES (?1, ?2);",
            params![short, game.name],
        )?;
        let gameid = self.conn.last_insert_rowid();

        log::info!("game {} -> ID {}", short, gameid);

        let segment_ids = self.add_segments(game)?;

        for (short, category) in &game.categories {
            self.add_category(gameid, short, category, &segment_ids)?;
        }

        Ok(())
    }

    fn add_category(
        &self,
        gameid: i64,
        short: &str,
        cat: &game::Category,
        segment_ids: &HashMap<String, i64>,
    ) -> Result<()> {
        log::info!("adding category {} for game ID {}", short, gameid);

        self.conn.execute(
            "INSERT INTO category (short, name) VALUES (?1, ?2);",
            params![short, cat.name],
        )?;
        let catid = self.conn.last_insert_rowid();

        log::info!("category {} -> ID {}", short, catid);

        self.conn.execute(
            "INSERT INTO game_category (gameid, categoryid) VALUES (?1, ?2);",
            params![gameid, catid],
        )?;

        for (position, segid) in category_segment_ids(cat, segment_ids)?.iter().enumerate() {
            self.conn.execute(
                "INSERT INTO category_segment (categoryid, segmentid, position) VALUES (?1, ?2, ?3);"
            , params![catid, segid, position])?;
        }

        Ok(())
    }

    fn add_segments(&self, game: &game::Game) -> Result<HashMap<String, i64>> {
        game.groups
            .iter()
            .map(|(short, segment)| Ok((short.clone(), self.add_segment(short, segment)?)))
            .collect()
    }

    fn add_segment(&self, short: &str, seg: &game::Group) -> Result<i64> {
        log::info!("adding segment {} ('{}')", short, seg.name);

        self.conn.execute(
            "INSERT INTO segment (short, name) VALUES (?1, ?2);",
            params![short, seg.name],
        )?;
        let segid = self.conn.last_insert_rowid();

        log::info!("segment {} -> ID {}", short, segid);

        for (position, split) in seg.splits.iter().enumerate() {
            self.add_split(segid, position, split)?;
        }

        Ok(segid)
    }

    fn add_split(&self, segid: i64, position: usize, split: &game::Split) -> Result<()> {
        log::info!("adding split '{}' for segment ID {}", split.name, segid);

        self.conn
            .execute("INSERT INTO split (name) VALUES (?1);", params![split.name])?;
        let splitid = self.conn.last_insert_rowid();

        self.conn.execute(
            "INSERT INTO segment_split (segmentid, splitid, position) VALUES (?1, ?2, ?3);",
            params![segid, splitid, position],
        )?;

        Ok(())
    }

    /// Initialises a session for the short-named `game` and `category`.
    ///
    /// # Errors
    ///
    /// Fails if there is no such category for the given game in the database,
    /// or the game doesn't exist, or any other database error occurs.
    pub fn init_session(&self, game: &str, category: &str) -> Result<Session> {
        let metadata = self.get_metadata(game, category)?;
        let run = self.init_run(metadata.category_id)?;
        Ok(Session::new(metadata, run))
    }

    fn get_metadata(&self, game: &str, cat: &str) -> Result<Metadata> {
        self.conn.query_row_and_then(
            "
            SELECT category.id, game.name, category.name
            FROM       game
            INNER JOIN game_category ON (game.id                  = game_category.gameid)
            INNER JOIN category      ON (game_category.categoryid = category.id         )
            WHERE game.short     = ?1
              AND category.short = ?2
        ;",
            params![game, cat],
            |row| {
                Ok(Metadata {
                    category_id: row.get(0)?,
                    game: row.get(1)?,
                    category: row.get(2)?,
                })
            },
        )
    }

    fn init_run(&self, category_id: i64) -> Result<Run> {
        let attempt = self.conn.query_row(
            "
        SELECT count(id)
        FROM run
        WHERE categoryid = ?1
        ;",
            params![category_id],
            |row| row.get(0),
        )?;
        let splits = self.init_splits(category_id)?;
        Ok(Run { attempt, splits })
    }

    fn init_splits(&self, category_id: i64) -> Result<Vec<Split>> {
        // TODO(@MattWindsor91): get the segments too.
        self.conn
            .prepare(
                "
        SELECT split.name
        FROM       split
        INNER JOIN segment_split    ON (split.id                = segment_split.splitid     )
        INNER JOIN category_segment ON (segment_split.segmentid = category_segment.segmentid)
        WHERE category_segment.categoryid = ?1
        ORDER BY category_segment.position ASC
               , segment_split.position    ASC
        ;",
            )?
            .query_and_then(params![category_id], |row| {
                Ok(Split::new(&row.get::<_, String>(0)?))
            })?
            .collect()
    }
}

fn category_segment_ids(
    cat: &game::Category,
    segment_ids: &HashMap<String, i64>,
) -> Result<Vec<i64>> {
    cat.groups
        .iter()
        .map(|short| {
            segment_ids
                .get(short)
                .copied()
                .ok_or_else(|| Error::NoPrimaryKey(short.clone()))
        })
        .collect()
}
