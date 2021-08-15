/*! The top-level UI module.

This UI presents, and allows in-flight modifications to, run attempts.  It can
be attached to the database to allow finished attempts to be committed.

The UI itself has a roughly model-view-presenter layout (with the downstream
attempt session forming the model).
*/

pub mod presenter;
pub mod view;

pub use presenter::Presenter;
pub use view::View;

/// Runs the user interface, configured by `cfg`, over `session`.
///
/// # Errors
///
/// Propagates any errors from creating, spawning, or running the view.
pub fn run(cfg: view::Config, session: crate::model::attempt::Session) -> view::Result<()> {
    let p = Presenter::new(session);
    View::new(cfg)?.spawn(p)?.run()?;
    Ok(())
}
