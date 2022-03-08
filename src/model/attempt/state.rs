/*! Session state.
 *
 * This is mainly exported outside of the session for the purposes of dumping a
 */

use super::{super::super::model::timing::Comparison, Run};

/// The state of a session.
///
/// The session state contains both a run and its current comparison.
/// A session's state can be dumped out at any point.
#[derive(Clone, Debug)]
pub struct State {
    pub run: Run,
    /// Comparison data for the game/category currently being run.
    pub comparison: Comparison,
}
