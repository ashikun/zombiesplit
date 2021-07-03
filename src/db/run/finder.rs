//! The [Finder] struct and related data.

use chrono::TimeZone;
use rusqlite::{named_params, Connection, Statement};

use crate::{
    db::category::GcID,
    model::history::{self, RunSummary, TimeSummary},
};

use super::super::error::{Error, Result};

/// Object for finding historic runs of interest in the database.
pub struct Finder<'conn> {
    /// Query used for summarising all runs logged on a game-category.
    query_summary: Statement<'conn>,
}

impl<'conn> Finder<'conn> {
    /// Constructs a run finder.
    ///
    /// # Errors
    ///
    /// Errors if the database can't prepare a query.
    pub fn new(conn: &'conn Connection) -> Result<Self> {
        Ok(Self {
            query_summary: conn.prepare(SQL_SUMMARY)?,
        })
    }

    /// Gets summaries for each run on a given game-category ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn runs_for(&mut self, id: GcID) -> Result<Vec<history::RunSummary<GcID>>> {
        self.query_summary
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
}

fn date_from_timestamp(timestamp: i64) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(Error::BadRunTimestamp(timestamp))
}

const SQL_SUMMARY: &str = "
SELECT is_completed
     , run.timestamp
     , SUM(time_ms) total
     , (CASE
        WHEN is_completed = 1
        THEN (RANK() OVER (PARTITION BY game_category_id, is_completed ORDER BY SUM(time_ms)))
        ELSE NULL
        END
       ) AS rank
FROM
    run
    INNER JOIN run_split      USING (run_id)
    INNER JOIN run_split_time USING (run_split_id)
WHERE
    game_category_id = :game_category
GROUP BY
    run_id
ORDER BY
    rank ASC NULLS LAST,
    run.timestamp ASC
;";
