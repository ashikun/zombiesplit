//! Implements run observation by autosaving runs into the database on reset.

use std::rc::Rc;

use log::{info, warn};

use crate::{
    db::Result,
    model::{attempt, game::category::ShortDescriptor, history},
    Db,
};

/// An observer that logs runs into a database.
pub struct Observer {
    /// A reference to the database.
    db: Rc<Db>,
}

impl attempt::Observer for Observer {
    fn observe(&self, evt: attempt::observer::Event) {
        if let attempt::observer::Event::Reset(run) = evt {
            log_err(self.try_save_run(run));
        }
    }
}

impl Observer {
    #[must_use]
    pub fn new(db: Rc<Db>) -> Self {
        Observer { db }
    }

    /// Shortcut for creating a boxed database observer.
    #[must_use]
    pub fn boxed(db: Rc<Db>) -> Box<dyn attempt::Observer> {
        Box::new(Self::new(db))
    }

    fn try_save_run(&self, run: Option<history::run::FullyTimed<ShortDescriptor>>) -> Result<()> {
        if let Some(run) = run {
            self.db.add_run(&run)?;
            info!("saved run at {}", run.date);
        } else {
            info!("skipped saving run -- not started");
        }
        Ok(())
    }
}

fn log_err(e: Result<()>) {
    e.unwrap_or_else(|e| warn!("error saving run: {}", e));
}
