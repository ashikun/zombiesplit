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

/// A manager of a zombiesplit server.
///
/// This holds the configuration and database handles that will be used by the server proper.
pub struct Manager<'c> {
    cfg: config::System<'c>,
    db: Rc<db::Db>,
    reader: db::Reader,
}

impl<'c> Manager<'c> {
    /// Constructs a new server, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::System<'c>) -> Result<Self> {
        let db = Rc::new(db::Db::new(&cfg.db_path)?);
        let reader = db.reader()?;
        Ok(Self { cfg, db, reader })
    }

    /// Creates a server for the given game.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn run(&self, desc: &ShortDescriptor) -> Result<Server> {
        let insp = self.reader.inspect(desc)?;

        let mut session = self.session(insp)?;

        let db_obs: Rc<dyn attempt::Observer> = Rc::new(db::Observer::new(self.db.clone()));
        session.observers.add(Rc::downgrade(&db_obs));

        let debug_obs: Rc<dyn attempt::Observer> = Rc::new(Debug);
        session.observers.add(Rc::downgrade(&debug_obs));

        Ok(Server {
            db_obs,
            debug_obs,
            session,
        })
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

/// A server, wrapping a session with the means to control it.
///
/// A server owns a running session, as well as the various observers attached to it, and performs
/// many of the tasks of bringing up, maintaining, and tearing down those elements.
///
/// In future, a server will also expose its control plane to clients.
pub struct Server<'cmp> {
    // TODO(@MattWindsor91): de-public all of these
    pub db_obs: Rc<dyn attempt::Observer>,
    pub debug_obs: Rc<dyn attempt::Observer>,

    /// The session being wrapped by this server.
    pub session: attempt::Session<'cmp>,
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
