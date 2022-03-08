//! Implements run sinking by autosaving runs into the database on reset.

use std::rc::Rc;

use log::info;

use crate::{model::session::sink, Db};

// TODO(@MattWindsor91): can we eliminate this?

/// A run sink that logs runs into a database.
#[derive(Clone)]
pub struct Sink {
    /// A reference to the database.
    db: Rc<Db>,
}

impl sink::Sink for Sink {
    fn accept(&mut self, run: sink::Run) -> sink::Result {
        self.db.add_run(&run).map_err(anyhow::Error::new)?;
        info!("saved run at {}", run.date);
        Ok(sink::Outcome::Saved)
    }
}

impl Sink {
    #[must_use]
    pub fn new(db: Rc<Db>) -> Self {
        Sink { db }
    }
}
