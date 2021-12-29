//! High-level database inspection queries.

use std::iter::repeat;

use super::{
    category::{self, id::InfoWithID},
    run,
    util::WithID,
    Result,
};
use crate::model::{aggregate, attempt, comparison, history, short, Time};

/// Inspects various aspects of the database for a given game-category ID.
pub struct Inspector<'db> {
    /// The game-category being inspected.
    pub info: InfoWithID,

    // TODO(@MattWindsor91): abstract the getters into traits, so we can mock
    // the inspector logic.
    /// The run getter.
    pub run: run::Getter<'db>,
    /// The category getter.
    pub cat: category::Getter<'db>,
}

impl<'db> AsMut<run::Getter<'db>> for Inspector<'db> {
    fn as_mut(&mut self) -> &mut run::Getter<'db> {
        &mut self.run
    }
}

impl<'db> AsMut<category::Getter<'db>> for Inspector<'db> {
    fn as_mut(&mut self) -> &mut category::Getter<'db> {
        &mut self.cat
    }
}

impl<'db> comparison::Provider for Inspector<'db> {
    fn comparison(&mut self) -> Option<comparison::Comparison> {
        // TODO(@MattWindsor91): do something with these errors.
        self.comparison_inner().ok()
    }
}

impl<'db> Inspector<'db> {
    /// Initialises an attempt session for the game/category referred to by
    /// `desc`, and with the given observer.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn init_session<'obs, O: attempt::Observer>(
        &mut self,
        obs: &'obs O,
    ) -> Result<attempt::Session<'db, 'obs, O>> {
        Ok(attempt::Session::new(
            self.info.info.clone(),
            self.cat.run(&self.info)?,
            obs,
        ))
    }

    /// Gets the run for this game-category pair.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_pb(
        &mut self,
        level: history::timing::Level,
    ) -> Result<Option<history::run::ForLevel<category::GcID>>> {
        self.run
            .run_pb_for(self.info.id)?
            .map(|x| self.add_timing(x, level))
            .transpose()
    }

    fn add_timing(
        &mut self,
        run: WithID<history::run::Summary<category::GcID>>,
        level: history::timing::Level,
    ) -> Result<history::run::ForLevel<category::GcID>> {
        match level {
            history::timing::Level::Summary => {
                Ok(run.item.map_timing(history::timing::ForLevel::from))
            }
            history::timing::Level::Totals => Ok(self
                .add_split_totals(run)?
                .item
                .map_timing(history::timing::ForLevel::from)),
            history::timing::Level::Full => todo!("full timing not yet implemented"),
        }
    }

    /// Adds split totals to an existing run.
    ///
    /// # Errors
    ///
    /// Returns any errors from querying the split totals.
    fn add_split_totals(
        &mut self,
        run: WithID<history::run::Summary<category::GcID>>,
    ) -> Result<WithID<history::run::WithTotals<category::GcID>>> {
        let totals = self.run.split_totals_for(run.id)?;
        Ok(run.map_item(|i| i.with_timing(totals)))
    }

    fn comparison_inner(&mut self) -> Result<comparison::Comparison> {
        let split_pbs = self.run.split_pbs_for(self.info.id)?;
        let run_pb = self.run_pb_with_totals()?;

        let splits = self.cat.splits(&self.info.id)?;
        let split_pbs = all_splits_with_pbs(&split_pbs, &splits);
        let run_pbs = in_run_iter(run_pb, &splits);

        Ok(split_pbs
            .zip(run_pbs)
            .map(|((short, split_pb), in_run)| (short, comparison::Split { split_pb, in_run }))
            .collect())
    }

    fn run_pb_with_totals(&mut self) -> Result<Option<history::run::WithTotals<category::GcID>>> {
        self.run
            .run_pb_for(self.info.id)?
            .map(|x| self.add_split_totals(x).map(|x| x.item))
            .transpose()
    }
}

fn all_splits_with_pbs<'a>(
    pbs: &'a short::Map<Time>,
    splits: &'a attempt::split::Set,
) -> impl Iterator<Item = (short::Name, Option<Time>)> + 'a {
    splits
        .iter()
        .map(move |s| (s.info.short, pbs.get(&s.info.short).copied()))
}

/// Gets an iterator over all of the in-run comparisons in a PB.
fn in_run_iter<'a>(
    pb: Option<history::run::WithTotals<category::GcID>>,
    splits: &'a attempt::split::Set,
) -> Box<dyn Iterator<Item = Option<aggregate::Set>> + 'a> {
    // TODO(@MattWindsor91): decouple this for testing.
    pb.map_or_else(
        || empty_splits(splits),
        |pb| {
            Box::new(
                aggregate::Set::accumulate_pairs(splits.iter().map(move |s| {
                    (
                        s.info.short,
                        pb.timing
                            .totals
                            .get(&s.info.short)
                            .copied()
                            .unwrap_or_default(),
                    )
                }))
                .map(|(_, x)| Option::Some(x)),
            )
        },
    )
}

fn empty_splits(splits: &attempt::split::Set) -> Box<dyn Iterator<Item = Option<aggregate::Set>>> {
    Box::new(repeat(None).take(splits.len()))
}
