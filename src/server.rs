//! The zombiesplit server.

use std::rc::Rc;

use super::{
    config,
    db::{self, inspect::Inspector},
    model::{
        self,
        attempt::{self, observer::Debug},
        comparison,
        game::category::ShortDescriptor,
    },
    ui,
};
use thiserror::Error;

/// An instance of a zombiesplit server.
///
/// A server owns a running session, as well as the various observers attached to it, and performs
/// many of the tasks of bringing up, maintaining, and tearing down those elements.
///
/// In future, a server will also expose its control plane to clients.
pub struct Server<'c> {
    cfg: config::System<'c>,
    db: Rc<db::Db>,
}

impl<'c> Server<'c> {
    /// Constructs a new server, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::System<'c>) -> Result<Self> {
        let db = Rc::new(db::Db::new(&cfg.db_path)?);
        Ok(Self { cfg, db })
    }

    /// Opens a split UI session for the given game/category descriptor.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn run(self, desc: &ShortDescriptor) -> Result<()> {
        let handle = self.db.reader()?;
        let insp = handle.inspect(desc)?;

        let mut session = self.session(insp)?;

        let db_obs: Rc<dyn attempt::Observer> = Rc::new(db::Observer::new(self.db));
        session.observers.add(Rc::downgrade(&db_obs));

        let debug_obs: Rc<dyn attempt::Observer> = Rc::new(Debug);
        session.observers.add(Rc::downgrade(&debug_obs));

        ui::run(self.cfg.ui, &mut session)?;
        Ok(())
    }

    fn session<'a>(&self, mut insp: Inspector<'a>) -> Result<model::attempt::Session<'a>> {
        let mut session = insp.init_session()?;
        session.set_comparison_provider(self.comparison_provider(insp));
        Ok(session)
    }

    fn comparison_provider<'a>(&self, insp: Inspector<'a>) -> Box<dyn comparison::Provider + 'a> {
        match self.cfg.comparison_provider {
            config::system::ComparisonProvider::Database => Box::new(insp),
            _ => Box::new(comparison::NullProvider),
        }
    }
}

/// The top-level server error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("database error")]
    Db(#[from] db::Error),
    #[error("UI error")]
    View(#[from] ui::Error),
}

/// The top-level server result type.
pub type Result<T> = std::result::Result<T, Error>;
