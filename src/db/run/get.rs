//! The [Getter] struct and related data.

use chrono::TimeZone;
use rusqlite::{named_params, Connection, Statement};

use crate::model::{history, short, timing::time};

use super::super::{
    category::GcID,
    error::{Error, Result},
    util::WithID,
};

/// Object for finding historic runs of interest in the database.
pub struct Getter<'conn> {
    /// Query used for getting a run by its number.
    query_run_at_index: Statement<'conn>,
    /// Query used for summarising all runs logged on a game-category.
    query_all_runs: Statement<'conn>,
    /// Query used for finding all split totals for a run.
    query_splits_for_run: Statement<'conn>,
}

impl<'conn> Getter<'conn> {
    /// Constructs a run finder.
    ///
    /// # Errors
    ///
    /// Errors if the database can't prepare a query.
    pub fn new(conn: &'conn Connection) -> Result<Self> {
        Ok(Self {
            query_run_at_index: conn.prepare(SQL_RUN_AT_INDEX)?,
            query_all_runs: conn.prepare(SQL_ALL_RUNS)?,
            query_splits_for_run: conn.prepare(SQL_SPLITS_FOR_RUN)?,
        })
    }

    /// Gets the run at index `index` within the game-category at database ID `id`.
    ///
    /// # Errors
    ///
    /// Errors if the run can't be converted from a database row.
    pub fn run_at(
        &mut self,
        id: GcID,
        index: usize,
    ) -> Result<Option<WithID<history::run::Summary<GcID>>>> {
        self.query_run_at_index
            .query_and_then(named_params![":game_category": id, ":index": index], |r| {
                WithID::from_row(id, r)
            })?
            .next()
            .transpose()
    }

    /// Gets summaries for each run on a given game-category ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn runs_for(&mut self, id: GcID) -> Result<Vec<WithID<history::run::Summary<GcID>>>> {
        self.query_all_runs
            .query_and_then(named_params![":game_category": id], |r| {
                WithID::from_row(id, r)
            })?
            .collect()
    }

    /// Gets split totals for the run with the given ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn split_totals_for(&mut self, id: i64) -> Result<history::timing::Totals> {
        let totals = self
            .query_splits_for_run
            .query_and_then(named_params![":run": id], |r| {
                let short: short::Name = r.get("short")?;
                let total: time::Time = r.get("total")?;
                Ok((short, total))
            })?
            .collect::<Result<short::Map<time::Time>>>()?;
        Ok(history::timing::Totals { totals })
    }

    /// Adds split totals to an existing run.
    ///
    /// # Errors
    ///
    /// Returns any errors from querying the split totals.
    pub fn add_split_totals(
        &mut self,
        run: WithID<history::run::Summary<GcID>>,
    ) -> Result<WithID<history::run::WithTotals<GcID>>> {
        let totals = self.split_totals_for(run.id)?;
        Ok(run.map_item(|i| i.with_timing(totals)))
    }
}

fn date_from_timestamp(timestamp: i64) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(Error::BadRunTimestamp(timestamp))
}

impl WithID<history::run::Summary<GcID>> {
    /// Constructs a summary from a row of one of the run-summary queries.
    ///
    /// This depends on the queries returning the same thing.
    ///
    /// # Errors
    ///
    /// Errors if any of the fields are missing or inconvertible.
    pub fn from_row(gcid: GcID, r: &rusqlite::Row) -> Result<Self> {
        Ok(Self {
            id: r.get("run_id")?,
            item: history::run::Summary {
                category_locator: gcid,
                date: date_from_timestamp(r.get("date")?)?,
                was_completed: r.get("is_completed")?,
                timing: history::timing::Summary {
                    total: r.get("total")?,
                    rank: r.get("rank")?,
                },
            },
        })
    }
}

const SQL_RUN_AT_INDEX: &str = "
SELECT run_id
     , is_completed
     , run.timestamp AS date
     , SUM(time_ms)  AS total
     , (CASE
        WHEN is_completed = 1
        THEN (RANK() OVER (PARTITION BY game_category_id, is_completed ORDER BY SUM(time_ms)))
        ELSE NULL
        END
       ) AS rank
  FROM run
       INNER JOIN run_split      USING (run_id)
       INNER JOIN run_split_time USING (run_split_id)
 WHERE game_category_id = :game_category
 GROUP BY run_id
 ORDER BY run.timestamp
 LIMIT 1
OFFSET :index;";

const SQL_ALL_RUNS: &str = "
SELECT run_id
     , is_completed
     , run.timestamp AS date
     , SUM(time_ms)  AS total
     , (CASE
        WHEN is_completed = 1
        THEN (RANK() OVER (PARTITION BY game_category_id, is_completed ORDER BY SUM(time_ms)))
        ELSE NULL
        END
       ) AS rank
 FROM run
      INNER JOIN run_split      USING (run_id)
      INNER JOIN run_split_time USING (run_split_id)
WHERE game_category_id = :game_category
GROUP BY run_id
ORDER BY rank ASC NULLS LAST, run.timestamp ASC
;";

const SQL_SPLITS_FOR_RUN: &str = "
SELECT s.short AS short, total
  FROM run_split_total
       INNER JOIN run_split              USING (run_split_id)
       INNER JOIN run              AS r  USING (run_id)
       INNER JOIN segment_split          USING (split_id)
       INNER JOIN category_segment AS cs USING (segment_id)
       INNER JOIN game_category    AS gc USING (game_category_id)
       INNER JOIN split            AS s  USING (split_id)
 WHERE run_id = :run
   AND cs.category_id = gc.category_id
    -- this fixes an ambiguity in the current database schema
    -- where split->category pulls in categories other than that of the run.
 ORDER BY cs.position ASC, segment_split.position ASC;";
