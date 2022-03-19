//! High-level database inspection queries.

use super::{
    super::model::{history, session, timing},
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
    /// `desc`, and with the given observer and sink.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the database.
    pub fn init_session<'obs, O: session::Observer>(
        &mut self,
        obs: &'obs O,
    ) -> Result<session::Session<'db, 'obs, O>> {
        // TODO(@MattWindsor91): remove this?
        Ok(session::Session::new(self.cat.run(&self.info)?, obs))
    }

    /// Gets the run at the given index (ordered by timestamp) for this game-category pair.
    ///
    /// Indices start at zero.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_at_index<L: TimingLevel>(
        &mut self,
        index: usize,
        level: &L,
    ) -> Result<Option<history::run::Run<category::GcID, L::Output>>> {
        let run = self.run.run_at(self.info.id, index)?;
        self.lift_run(run, level)
    }

    /// Gets the personal-best run for this game-category pair.
    ///
    /// # Errors
    ///
    /// Returns any database errors occurring during the listing.
    pub fn run_pb<L: TimingLevel>(
        &mut self,
        level: &L,
    ) -> Result<Option<history::run::Run<category::GcID, L::Output>>> {
        let run = self.comparison.run_pb(self.info.id)?;
        self.lift_run(run, level)
    }

    fn lift_run<L: TimingLevel>(
        &mut self,
        raw_run: Option<WithID<history::run::Summary<category::GcID>>>,
        level: &L,
    ) -> Result<Option<history::run::Run<category::GcID, L::Output>>> {
        raw_run
            .map(|x| level.add_timing(self, x))
            .transpose()
            .map(|x| x.map(|x| x.item))
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

/// Trait implemented by types providing timing levels for run fetching.
///
/// This generalises `history::timing::Level` by making it possible to get a precise output type
/// we get from adding the typing information.
pub trait TimingLevel {
    /// Type of resulting timing information.
    type Output;

    /// Adds timing to a summary run using the services provided by `inspector`.
    ///
    /// # Errors
    ///
    /// May fail if there is a problem getting additional information from the database to populate
    /// the new timing information.
    fn add_timing(
        &self,
        inspector: &mut Inspector,
        run: WithID<history::run::Summary<category::GcID>>,
    ) -> Result<WithID<history::run::Run<category::GcID, Self::Output>>>;
}

/// Timing level for summaries.
///
/// This signals to the inspector that no extra timing information is required.
pub struct Summary;

impl TimingLevel for Summary {
    type Output = history::timing::Summary;

    fn add_timing(
        &self,
        _inspector: &mut Inspector,
        run: WithID<history::run::Summary<category::GcID>>,
    ) -> Result<WithID<history::run::Summary<category::GcID>>> {
        Ok(run)
    }
}

impl TimingLevel for history::timing::Level {
    type Output = history::timing::ForLevel;

    fn add_timing(
        &self,
        inspector: &mut Inspector,
        run: WithID<history::run::Summary<category::GcID>>,
    ) -> Result<WithID<history::run::ForLevel<category::GcID>>> {
        match self {
            history::timing::Level::Summary => {
                Ok(run.map_item(|i| i.map_timing(history::timing::ForLevel::from)))
            }
            history::timing::Level::Totals => Ok(inspector
                .add_split_totals(run)?
                .map_item(|i| i.map_timing(history::timing::ForLevel::from))),
            history::timing::Level::Full => todo!("full timing not yet implemented"),
        }
    }
}
