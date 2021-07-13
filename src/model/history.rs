/*!
Models related to finished ('historic') artefacts.

These models are useful for transferring run information into and out of
flat files, as well as storing finished runs into the database.
*/

pub mod run;
pub mod timing;

pub use run::Run;
