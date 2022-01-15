//! Database functionality for loading and storing games.

use std::collections::HashMap;

use rusqlite::{named_params, Transaction};

use super::error::{Error, Result};
use crate::model::{game, short};

/// Inserts one or more games into the database.
pub(super) struct Inserter<'conn, 'tx> {
    /// The underlying database transaction.
    tx: &'tx Transaction<'conn>,
    /// The ID of the current game.
    game_id: i64,
    /// Maps every split shortname stored into the database to its primary key.
    split_ids: short::Map<i64>,
    /// Maps every segment shortname stored into the database to its primary key.
    segment_ids: short::Map<i64>,
    /// Prepared queries.
    queries: HashMap<Query, rusqlite::Statement<'tx>>,
}

const SQL_GAME: &str = "INSERT INTO game (short, name) VALUES (:short, :name);";
const SQL_CATEGORY: &str = "INSERT INTO category (short, name) VALUES (:short, :name);";
const SQL_SEGMENT: &str = "INSERT INTO segment (short, name) VALUES (:short, :name);";
const SQL_SPLIT: &str = "INSERT INTO split (short, name) VALUES (:short, :name);";
const SQL_GAME_CATEGORY: &str =
    "INSERT INTO game_category (game_id, category_id) VALUES (:game_id, :category_id);";
const SQL_CATEGORY_SEGMENT: &str = "INSERT INTO category_segment (category_id, segment_id, position) VALUES (:category_id, :segment_id, :position);";
const SQL_SEGMENT_SPLIT: &str = "INSERT INTO segment_split (segment_id, split_id, position) VALUES (:segment_id, :split_id, :position);";

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum Query {
    Game,
    Category,
    Segment,
    Split,
    GameCategory,
    CategorySegment,
    SegmentSplit,
}

const SQL: &[(Query, &str)] = &[
    (Query::Game, SQL_GAME),
    (Query::Category, SQL_CATEGORY),
    (Query::Segment, SQL_SEGMENT),
    (Query::Split, SQL_SPLIT),
    (Query::GameCategory, SQL_GAME_CATEGORY),
    (Query::CategorySegment, SQL_CATEGORY_SEGMENT),
    (Query::SegmentSplit, SQL_SEGMENT_SPLIT),
];

/// A transaction that is inserting one or more games into the database.
impl<'conn, 'tx> Inserter<'conn, 'tx> {
    /// Constructs an inserting transation.
    ///
    /// # Errors
    ///
    /// Raises an error if preparing the transaction or its queries fails.
    pub(super) fn new(tx: &'tx rusqlite::Transaction<'conn>) -> Result<Self> {
        let mut result = Self {
            tx,
            game_id: 0,
            split_ids: short::Map::new(),
            segment_ids: short::Map::new(),
            queries: HashMap::new(),
        };
        result.prepare_queries()?;
        Ok(result)
    }

    fn prepare_queries(&mut self) -> Result<()> {
        for (key, sql) in SQL {
            let value = self.tx.prepare(sql)?;
            self.queries.insert(*key, value);
        }
        Ok(())
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

    fn query(&mut self, query: Query) -> &mut rusqlite::Statement<'tx> {
        self.queries
            .get_mut(&query)
            .expect("internal error: query missing")
    }

    fn add_main(&mut self, short: &str, game: &game::Config) -> Result<()> {
        log::info!("adding game {short}");

        self.query(Query::Game)
            .execute(named_params![":short": short, ":name": game.name])?;
        self.game_id = self.tx.last_insert_rowid();

        log::info!("game {short} -> ID {}", self.game_id);
        Ok(())
    }

    fn add_splits(&mut self, game: &game::Config) -> Result<()> {
        for (short, split) in &game.splits {
            self.add_split(*short, split)?;
        }
        Ok(())
    }

    fn add_split(&mut self, short: short::Name, split: &game::config::Split) -> Result<()> {
        log::info!("adding split {short} ('{}')", split.name);

        self.query(Query::Split)
            .execute(named_params![":short": short, ":name": split.name])?;

        let split_id = self.tx.last_insert_rowid();
        log::info!("split {short} -> ID {split_id}");
        self.split_ids.insert(short, split_id);

        Ok(())
    }

    fn add_segments(&mut self, game: &game::Config) -> Result<()> {
        for (short, seg) in &game.segments {
            self.add_segment(*short, seg)?;
        }
        Ok(())
    }

    fn add_segment(&mut self, short: short::Name, segment: &game::config::Segment) -> Result<()> {
        let segment_id = self.add_segment_main(short, segment)?;
        self.add_splits_to_segment(segment_id, segment)
    }

    fn add_segment_main(
        &mut self,
        short: short::Name,
        segment: &game::config::Segment,
    ) -> Result<i64> {
        log::info!("adding segment {short} ('{}')", segment.name);
        self.query(Query::Segment)
            .execute(named_params![":short": short, ":name": segment.name])?;

        let segment_id = self.tx.last_insert_rowid();
        log::info!("segment {short} -> ID {segment_id}");
        self.segment_ids.insert(short, segment_id);

        Ok(segment_id)
    }

    fn add_splits_to_segment(
        &mut self,
        segment_id: i64,
        segment: &game::config::Segment,
    ) -> Result<()> {
        for (position, split_id) in self.segment_split_ids(segment)?.iter().enumerate() {
            log::info!("adding split ID {} for segment {}", split_id, segment.name);
            self.query(Query::SegmentSplit).execute(named_params![
                ":segment_id": segment_id,
                ":split_id": split_id,
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
                        short: *short,
                        in_segment: segment.name.clone(),
                    })
            })
            .collect()
    }

    fn add_categories(&mut self, game: &game::Config) -> Result<()> {
        for (short, category) in &game.categories {
            self.add_category(*short, category)?;
        }

        Ok(())
    }

    fn add_category(
        &mut self,
        short: short::Name,
        category: &game::config::Category,
    ) -> Result<()> {
        let category_id = self.add_category_main(short, category)?;
        self.add_category_to_game(category_id)?;
        self.add_segments_to_category(category_id, category)
    }

    fn add_category_main(
        &mut self,
        short: short::Name,
        category: &game::config::Category,
    ) -> Result<i64> {
        log::info!("adding category {} for game ID {}", short, self.game_id);
        self.query(Query::Category)
            .execute(named_params![":short": short, ":name": category.name])?;

        let categoryid = self.tx.last_insert_rowid();
        log::info!("category {short} -> ID {categoryid}");
        Ok(categoryid)
    }

    fn add_category_to_game(&mut self, category_id: i64) -> Result<()> {
        let game_id = self.game_id;
        self.query(Query::GameCategory).execute(named_params![
            ":game_id": game_id,
            ":category_id": category_id
        ])?;
        Ok(())
    }

    fn add_segments_to_category(
        &mut self,
        category_id: i64,
        category: &game::config::Category,
    ) -> Result<()> {
        for (position, segment_id) in self.category_segment_ids(category)?.iter().enumerate() {
            log::info!(
                "adding segment ID {segment_id} for category {}",
                category.name
            );
            self.query(Query::CategorySegment).execute(named_params![
                ":category_id": category_id,
                ":segment_id": segment_id,
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
                        short: *short,
                        in_category: category.name.clone(),
                    })
            })
            .collect()
    }
}
