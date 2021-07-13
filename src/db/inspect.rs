//! High-level database inspection queries.

use super::{category, run, util::WithID, Result};
use crate::model::history;

/// A zombiesplit database inspector.
///
/// This combines the various low-level database getters, and adds high-level
///
pub struct Inspector<'db> {
    // TODO(@MattWindsor91): abstract the getters into traits, so we can mock
    // the inspector logic.
    /// The run getter.
    pub run: run::Getter<'db>,
    /// The category getter.
    pub cat: category::Getter<'db>,
}

impl<'db> Inspector<'db> {
    /// Gets the run for the given game/category locator.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_pb(
        &mut self,
        loc: &impl category::Locator,
        level: history::timing::Level,
    ) -> Result<Option<history::run::ForLevel<category::GcID>>> {
        let id = loc.locate(&mut self.cat)?;

        if let Some(pb) = self.run.run_pb_for(id)? {
            Ok(Some(self.add_timing(pb, level)?))
        } else {
            Ok(None)
        }
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
    pub fn add_split_totals(
        &mut self,
        run: WithID<history::run::Summary<category::GcID>>,
    ) -> Result<WithID<history::run::WithTotals<category::GcID>>> {
        let totals = self.run.split_totals_for(run.id)?;
        Ok(run.map_item(|i| i.with_timing(totals)))
    }
}
