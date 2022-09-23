//! The [Getter] struct.

use super::{
    super::{category, category::GcID, error::Result, run, util::WithID},
    sql,
};
use crate::model::{
    history, session, short,
    timing::{aggregate, comparison, time::human, Comparison},
};
use rusqlite::{named_params, Connection, Statement};

/// Low-level interface for getting comparison data.
pub struct Getter<'conn> {
    /// SQL query for getting the personal-best run.
    run_pb_query: Statement<'conn>,
    /// SQL query for getting split personal-bests.
    split_pbs_query: Statement<'conn>,
    /// SQL query for getting the sum of best.
    sum_of_best_query: Statement<'conn>,
}

impl<'conn> Getter<'conn> {
    /// Constructs a comparison getter.
    ///
    /// # Errors
    ///
    /// Errors if the database can't prepare a query.
    pub fn new(conn: &'conn Connection) -> Result<Self> {
        Ok(Self {
            run_pb_query: conn.prepare(sql::RUN_PB)?,
            split_pbs_query: conn.prepare(sql::SPLIT_PBS)?,
            sum_of_best_query: conn.prepare(sql::SUM_OF_BEST)?,
        })
    }

    /// Gets a comparison for a game-category ID.
    ///
    /// We need the category getter to pull the split ordering for the game-category (so that we can
    /// get an ordering on the splits), and the run getter to pull details about the PB run.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn get(
        &mut self,
        cat_get: &mut category::Getter<'conn>,
        run_get: &mut run::Getter<'conn>,
        gcid: GcID,
    ) -> Result<Comparison> {
        let pb_summary = self.run_pb(gcid)?;

        let total_in_pb_run = pb_summary.as_ref().map(|x| x.item.timing.total);
        let sum_of_best = self.sum_of_best(gcid)?;
        let run = comparison::Run {
            total_in_pb_run,
            sum_of_best,
        };

        let pb_full = pb_summary
            .map(|x| run_get.add_split_totals(x))
            .transpose()?;

        let splits = cat_get.splits(&gcid)?;
        Ok(Comparison {
            splits: self.splits(gcid, &splits, pb_full)?,
            run,
        })
    }

    /// Gets the PB run for a game-category ID, if one exists.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn run_pb(&mut self, id: GcID) -> Result<Option<WithID<history::run::Summary<GcID>>>> {
        self.run_pb_query
            .query_and_then(named_params![":game_category": id], |r| {
                WithID::from_row(id, r)
            })?
            .next()
            .transpose()
    }

    /// Gets PBs for each split on a given game-category ID.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn split_pbs(&mut self, id: GcID) -> Result<short::Map<human::Time>> {
        self.split_pbs_query
            .query_and_then(named_params![":game_category": id], |row| {
                Ok((row.get("short")?, row.get("total")?))
            })?
            .collect()
    }

    /// Gets the sum-of-best for a game-category ID, if one exists.
    ///
    /// # Errors
    ///
    /// Errors if the database query fails.
    pub fn sum_of_best(&mut self, id: GcID) -> Result<Option<human::Time>> {
        Ok(self
            .sum_of_best_query
            .query_row(named_params![":game_category": id], |r| r.get("total"))?)
    }

    fn splits(
        &mut self,
        gcid: GcID,
        splits: &session::split::Set,
        pb_run: Option<WithID<history::run::WithTotals<GcID>>>,
    ) -> Result<short::Map<comparison::Split>> {
        let split_pbs = self.split_pbs(gcid)?;
        let run_pb_splits = pb_run.map_or_else(Default::default, |x| {
            aggregate(splits, x.item.timing.totals)
        });

        Ok(merge_split_data(splits, &split_pbs, &run_pb_splits))
    }
}

/// Lifts a split time map to one over aggregates by summing across the splits in `split`.
fn aggregate(
    splits: &session::split::Set,
    totals: short::Map<human::Time>,
) -> short::Map<aggregate::Set> {
    // TODO(@MattWindsor91): decouple this for testing.
    aggregate::Set::accumulate_pairs(splits.iter().map(move |s| {
        (
            s.info.short,
            totals.get(&s.info.short).copied().unwrap_or_default(),
        )
    }))
    .collect()
}

fn merge_split_data(
    splits: &session::split::Set,
    split_pbs: &short::Map<human::Time>,
    run_pb_splits: &short::Map<aggregate::Set>,
) -> short::Map<comparison::Split> {
    splits
        .iter()
        .filter_map(|x| {
            run_pb_splits.get(&x.info.short).copied().map(|in_run| {
                (
                    x.info.short,
                    // The split PB should _really_ exist if there is an in-run PB.
                    comparison::Split {
                        split_pb: split_pbs.get(&x.info.short).copied().unwrap_or_default(),
                        in_pb_run: in_run,
                    },
                )
            })
        })
        .collect()
}
