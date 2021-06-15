//! Database functionality for loading and storing games.

use rusqlite::named_params;

use super::error::{Error, Result};
use crate::model::game;

/// Inserts one or more games into the database.
pub struct Inserter<'a> {
    /// The underlying database.
    db: &'a super::Db,
    /// The ID of the current game.
    game_id: i64,
    /// Maps every split shortname stored into the database to its primary key.
    split_ids: game::config::ShortNameMap<i64>,
    /// Maps every segment shortname stored into the database to its primary key.
    segment_ids: game::config::ShortNameMap<i64>,
    /// Prepared query used to insert games.
    game_query: rusqlite::Statement<'a>,
    /// Prepared query used to insert categories.
    category_query: rusqlite::Statement<'a>,
    /// Prepared query used to insert games.
    segment_query: rusqlite::Statement<'a>,
    /// Prepared query used to insert games.
    split_query: rusqlite::Statement<'a>,
    /// Prepared query used to link categories to games.
    game_category_query: rusqlite::Statement<'a>,
    /// Prepared query used to link segments to categories.
    category_segment_query: rusqlite::Statement<'a>,
    /// Prepared query used to link splits to segments.
    segment_split_query: rusqlite::Statement<'a>,
}

const SQL_GAME: &str = "INSERT INTO game (short, name) VALUES (:short, :name);";
const SQL_CATEGORY: &str = "INSERT INTO category (short, name) VALUES (:short, :name);";
const SQL_SEGMENT: &str = "INSERT INTO segment (short, name) VALUES (:short, :name);";
const SQL_SPLIT: &str = "INSERT INTO split (short, name) VALUES (:short, :name);";
const SQL_GAME_CATEGORY: &str =
    "INSERT INTO game_category (gameid, categoryid) VALUES (:gameid, :categoryid);";
const SQL_CATEGORY_SEGMENT: &str = "INSERT INTO category_segment (categoryid, segmentid, position) VALUES (:categoryid, :segmentid, :position);";
const SQL_SEGMENT_SPLIT: &str = "INSERT INTO segment_split (segmentid, splitid, position) VALUES (:segmentid, :splitid, :position);";

impl<'a> Inserter<'a> {
    pub(super) fn new(db: &'a super::Db) -> Result<Self> {
        Ok(Self {
            db,
            game_id: 0,
            split_ids: game::config::ShortNameMap::new(),
            segment_ids: game::config::ShortNameMap::new(),

            game_query: db.conn.prepare(SQL_GAME)?,
            category_query: db.conn.prepare(SQL_CATEGORY)?,
            segment_query: db.conn.prepare(SQL_SEGMENT)?,
            split_query: db.conn.prepare(SQL_SPLIT)?,
            game_category_query: db.conn.prepare(SQL_GAME_CATEGORY)?,
            category_segment_query: db.conn.prepare(SQL_CATEGORY_SEGMENT)?,
            segment_split_query: db.conn.prepare(SQL_SEGMENT_SPLIT)?,
        })
    }

    /// Adds the game `game` to the database, assigning it shortname `short`.
    ///
    /// # Errors
    ///
    /// Raises an error if any of the SQL queries relating to inserting a game
    /// fail.
    pub fn add_game(&mut self, short: &str, game: &game::Config) -> Result<()> {
        self.add_main(short, game)?;
        self.add_splits(game)?;
        self.add_segments(game)?;
        self.add_categories(game)
    }

    fn add_main(&mut self, short: &str, game: &game::Config) -> Result<()> {
        log::info!("adding game {}", short);

        self.game_query
            .execute(named_params![":short": short, ":name": game.name])?;
        self.game_id = self.db.conn.last_insert_rowid();

        log::info!("game {} -> ID {}", short, self.game_id);
        Ok(())
    }

    fn add_splits(&mut self, game: &game::Config) -> Result<()> {
        for (short, split) in &game.splits {
            self.add_split(short, split)?;
        }
        Ok(())
    }

    fn add_split(&mut self, short: &str, split: &game::config::Split) -> Result<()> {
        log::info!("adding split {} ('{}')", short, split.name);

        self.split_query
            .execute(named_params![":short": short, ":name": split.name])?;

        let split_id = self.db.conn.last_insert_rowid();
        log::info!("split {} -> ID {}", short, split_id);
        self.split_ids.insert(short.to_owned(), split_id);

        Ok(())
    }

    fn add_segments(&mut self, game: &game::Config) -> Result<()> {
        for (short, seg) in &game.segments {
            self.add_segment(short, seg)?;
        }
        Ok(())
    }

    fn add_segment(&mut self, short: &str, segment: &game::config::Segment) -> Result<()> {
        let segment_id = self.add_segment_main(short, segment)?;
        self.add_splits_to_segment(segment_id, segment)
    }

    fn add_segment_main(&mut self, short: &str, segment: &game::config::Segment) -> Result<i64> {
        log::info!("adding segment {} ('{}')", short, segment.name);
        self.segment_query
            .execute(named_params![":short": short, ":name": segment.name])?;

        let segment_id = self.db.conn.last_insert_rowid();
        log::info!("segment {} -> ID {}", short, segment_id);
        self.segment_ids.insert(short.to_owned(), segment_id);

        Ok(segment_id)
    }

    fn add_splits_to_segment(
        &mut self,
        segment_id: i64,
        segment: &game::config::Segment,
    ) -> Result<()> {
        for (position, split_id) in self.segment_split_ids(segment)?.iter().enumerate() {
            log::info!("adding split ID {} for segment {}", split_id, segment.name);
            self.segment_split_query.execute(named_params![
                ":segmentid": segment_id,
                ":splitid": split_id,
                ":position": position
            ])?;
        }
        Ok(())
    }

    fn segment_split_ids(&self, segment: &game::config::Segment) -> Result<Vec<i64>> {
        segment
            .splits
            .iter()
            .map(|short| {
                self.split_ids
                    .get(short)
                    .copied()
                    .ok_or_else(|| Error::MissingSplit {
                        short: short.clone(),
                        in_segment: segment.name.clone(),
                    })
            })
            .collect()
    }

    fn add_categories(&mut self, game: &game::Config) -> Result<()> {
        for (short, category) in &game.categories {
            self.add_category(short, category)?;
        }

        Ok(())
    }

    fn add_category(&mut self, short: &str, category: &game::config::Category) -> Result<()> {
        let category_id = self.add_category_main(short, category)?;
        self.add_category_to_game(category_id)?;
        self.add_segments_to_category(category_id, category)
    }

    fn add_category_main(&mut self, short: &str, category: &game::config::Category) -> Result<i64> {
        log::info!("adding category {} for game ID {}", short, self.game_id);
        self.category_query
            .execute(named_params![":short": short, ":name": category.name])?;

        let categoryid = self.db.conn.last_insert_rowid();
        log::info!("category {} -> ID {}", short, categoryid);
        Ok(categoryid)
    }

    fn add_category_to_game(&mut self, category_id: i64) -> Result<()> {
        self.game_category_query
            .execute(named_params![":gameid": self.game_id, ":categoryid": category_id])?;
        Ok(())
    }

    fn add_segments_to_category(
        &mut self,
        category_id: i64,
        category: &game::config::Category,
    ) -> Result<()> {
        for (position, segment_id) in self.category_segment_ids(category)?.iter().enumerate() {
            log::info!(
                "adding segment ID {} for category {}",
                segment_id,
                category.name
            );
            self.category_segment_query.execute(named_params![
                ":categoryid": category_id,
                ":segmentid": segment_id,
                ":position": position
            ])?;
        }
        Ok(())
    }

    fn category_segment_ids(&self, category: &game::config::Category) -> Result<Vec<i64>> {
        category
            .segments
            .iter()
            .map(|short| {
                self.segment_ids
                    .get(short)
                    .copied()
                    .ok_or_else(|| Error::MissingSegment {
                        short: short.clone(),
                        in_category: category.name.clone(),
                    })
            })
            .collect()
    }
}
