//! High-level database inspection queries.

use crate::model::{attempt, history, timing};

use super::{
    category::{self, id::InfoWithID},
    comparison, run,
    util::WithID,
    Result,
};

/// Inspects various aspects of the database for a given game-category ID.
pub struct Inspector<'db> {
    /// The game-category being inspected.
    pub info: InfoWithID,

    // TODO(@MattWindsor91): abstract the getters into traits, so we can mock
    // the inspector logic.
    /// The run getter.
    pub run: run::Getter<'db>,
    /// The comparison getter.
    pub comparison: comparison::Getter<'db>,
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

impl<'db> timing::comparison::provider::Provider for Inspector<'db> {
    fn comparison(&mut self) -> timing::comparison::provider::Result {
        Ok(Some(
            self.comparison
                .get(&mut self.cat, &mut self.run, self.info.id)
                .map_err(anyhow::Error::from)?,
        ))
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
        self.comparison
            .run_pb(self.info.id)?
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
}
