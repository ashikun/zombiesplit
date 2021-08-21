//! The [Inserter] struct and related data.

use std::ops::Deref;

use log::info;
use rusqlite::{named_params, Connection, Statement};

use super::super::error::{Error, Result};
use crate::{
    db::category::GcID,
    model::{history, short, Time},
};

/// Object for inserting historic runs into to the database.
pub struct Inserter<'conn> {
    conn: &'conn dyn Deref<Target = Connection>,
    query_add_run: Statement<'conn>,
    query_add_split: Statement<'conn>,
    query_add_split_time: Statement<'conn>,
    query_split_shortmap: Statement<'conn>,
}

impl<'conn> Inserter<'conn> {
    /// Constructs a run adder.
    ///
    /// # Errors
    ///
    /// Errors if the database can't prepare a query.
    pub fn new<T: Deref<Target = Connection>>(conn: &'conn T) -> Result<Self> {
        Ok(Self {
            conn,
            query_add_run: conn.prepare(SQL_ADD_RUN)?,
            query_add_split: conn.prepare(SQL_ADD_SPLIT)?,
            query_add_split_time: conn.prepare(SQL_ADD_SPLIT_TIME)?,
            query_split_shortmap: conn.prepare(SQL_SPLIT_SHORTMAP)?,
        })
    }

    /// Adds a run to the database.
    ///
    /// The run must be using a game-category ID to identify the game and
    /// category pair.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn add(&mut self, run: &history::run::FullyTimed<GcID>) -> Result<()> {
        info!(
            "adding run from {} to game-category ID {}",
            run.date, run.category_locator.0
        );

        let run_id = self.add_main(run)?;
        let split_map = self.split_shortmap(run.category_locator)?;
        self.add_splits(run_id, &run.timing, &split_map)?;

        Ok(())
    }

    fn add_main<T>(&mut self, run: &history::Run<GcID, T>) -> Result<i64> {
        self.query_add_run.execute(named_params![
            ":is_completed": run.was_completed,
            ":timestamp": run.date.timestamp(),
            ":game_category": run.category_locator
        ])?;

        Ok(self.conn.last_insert_rowid())
    }

    fn split_shortmap(&mut self, id: GcID) -> Result<short::Map<i64>> {
        self.query_split_shortmap
            .query_and_then(named_params![":game_category": id], |row| {
                Ok((row.get(1)?, row.get(0)?))
            })?
            .collect()
    }

    fn add_splits(
        &mut self,
        run_id: i64,
        timing: &history::timing::Full,
        split_map: &short::Map<i64>,
    ) -> Result<()> {
        for (short, times) in &timing.times {
            // No point storing an empty split.
            if times.is_empty() {
                continue;
            }

            let split_id = split_map.get(short).ok_or_else(|| Error::MissingRunSplit {
                short: short.clone(),
            })?;
            self.query_add_split
                .execute(named_params![":run_id": run_id, ":split_id": split_id])?;
            let run_split_id = self.conn.last_insert_rowid();
            self.add_split_times(run_split_id, times)?;
        }
        Ok(())
    }

    fn add_split_times(&mut self, run_split_id: i64, times: &[Time]) -> Result<()> {
        for (position, time) in times.iter().enumerate() {
            self.query_add_split_time.execute(
                named_params![":run_split_id": run_split_id, ":position": position, ":time_ms": u32::from(*time)]
            )?;
        }
        Ok(())
    }
}

const SQL_ADD_RUN: &str = "
INSERT INTO run (is_completed, timestamp, game_category_id)
VALUES (:is_completed, :timestamp, :game_category);";

const SQL_ADD_SPLIT: &str = "
INSERT INTO run_split (run_id, split_id)
VALUES (:run_id, :split_id);";

const SQL_ADD_SPLIT_TIME: &str = "
INSERT INTO run_split_time (run_split_id, position, time_ms)
VALUES (:run_split_id, :position, :time_ms);";

// TODO(@MattWindsor91): similar to, but not quite, the one in category.
const SQL_SPLIT_SHORTMAP: &str = "
    SELECT split_id, split.short
    FROM split
            INNER JOIN segment_split    USING(split_id)
            INNER JOIN category_segment USING(segment_id)
            INNER JOIN game_category    USING(category_id)
    WHERE game_category_id = :game_category;";
