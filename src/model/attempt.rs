/*! Models relating to an in-progress attempt.

This module contains the bulk of the model surface of the zombiesplit server, covering:

- the representation of in-progress runs;
- sessions, which manage said runs and expose various API surfaces for handling them;
- actions, which form the command surface of sessions;
- observers, which form an observer pattern based API for monitoring changes to a session;
- sinks, which receive runs after the user resets the session.
*/
pub mod action;
pub mod observer;
pub mod run;
pub mod session;
pub mod sink;
pub mod split;

pub use action::Action;
pub use observer::Observer;
pub use run::Run;
pub use session::Session;
pub use sink::Sink;
pub use split::Split;
