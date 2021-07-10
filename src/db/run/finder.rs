//! The [Finder] struct and related data.

use chrono::TimeZone;
use rusqlite::{named_params, Connection, Statement};

use crate::{db::category::GcID, model::{Time, history::{self, RunSummary, TimeSummary}, short::Name}};

use super::super::error::{Error, Result};

/// Object for finding historic runs of interest in the database.
pub struct Finder<'conn> {
    /// Query used for summarising all runs logged on a game-category.
    query_all_runs: Statement<'conn>,
    /// Query used for finding the personal-best run for a game-category.
    query_comparison_run_pb: Statement<'conn>,
    /// Query used for finding the personal-best splits for a game-category.
    query_comparison_split_pbs: Statement<'conn>,
    /// Query used for finding all split totals for a run.
    query_splits_for_run: Statement<'conn>
}

impl<'conn> Finder<'conn> {
    /// Constructs a run finder.
    ///
    /// # Errors
    ///
    /// Errors if the database can't prepare a query.
    pub fn new(conn: &'conn Connection) -> Result<Self> {
        Ok(Self {
            query_all_runs: conn.prepare(SQL_ALL_RUNS)?,
            query_comparison_run_pb: conn.prepare(SQL_COMPARISON_RUN_PB)?,
            query_comparison_split_pbs: conn.prepare(SQL_COMPARISON_SPLIT_PBS)?,
            query_splits_for_run: conn.prepare(SQL_SPLITS_FOR_RUN)?
        })
    }

    /// Gets summaries for each run on a given game-category ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn runs_for(&mut self, id: GcID) -> Result<Vec<history::RunSummary<GcID>>> {
        self.query_all_runs
            .query_and_then(named_params![":game_category": id], |row| {
                Ok(RunSummary::<GcID> {
                    category_locator: id,
                    was_completed: row.get(0)?,
                    date: date_from_timestamp(row.get(1)?)?,
                    timing: TimeSummary {
                        total: row.get(2)?,
                        rank: row.get(3)?,
                    },
                })
            })?
            .collect()
    }

    /// Gets PBs for each split on a given game-category ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn split_pbs_for(&mut self, id: GcID) -> Result<Vec<(i64, Time)>> {
        self.query_comparison_split_pbs
            .query_and_then(named_params![":game_category": id], |row| {
                Ok((
                    row.get("split_id")?,
                    row.get("total")?,
                ))
            })?
            .collect()
    }
}

fn date_from_timestamp(timestamp: i64) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(Error::BadRunTimestamp(timestamp))
}

const SQL_ALL_RUNS: &str = "
SELECT is_completed
     , run.timestamp
     , SUM(time_ms) total
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

const SQL_COMPARISON_RUN_PB: &str = "
SELECT run_id
     , SUM(time_ms) AS total
  FROM run
       INNER JOIN run_split      USING (run_id)
       INNER JOIN run_split_time USING (run_split_id)
 WHERE game_category_id = :game_category
   AND is_completed = 1
 GROUP BY run_id
 ORDER BY total ASC
 LIMIT 1;";

const SQL_COMPARISON_SPLIT_PBS: &str = "
SELECT split_id
     , MIN(total) AS total
  FROM run_split_total
       INNER JOIN run_split        USING (run_split_id)
       INNER JOIN segment_split    USING (split_id)
       INNER JOIN category_segment USING (segment_id)
       INNER JOIN game_category    USING (category_id)
 WHERE game_category_id = :game_category
 GROUP BY split_id
 ORDER BY category_segment.position ASC, segment_split.position ASC;";

const SQL_SPLITS_FOR_RUN: &str = "
SELECT split_id, total
  FROM run_split_total
       INNER JOIN run_split              USING (run_split_id)
       INNER JOIN run              AS r  USING (run_id)
       INNER JOIN segment_split          USING (split_id)
       INNER JOIN category_segment AS cs USING (segment_id)
       INNER JOIN game_category    AS gc USING (game_category_id)
 WHERE run_id = :run_id
   AND cs.category_id = gc.category_id
    -- this fixes an ambiguity in the current database schema
    -- where split->category pulls in categories other than that of the run.
 ORDER BY cs.position ASC, segment_split.position ASC;";
