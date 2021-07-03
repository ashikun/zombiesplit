//! Implements run observation by autosaving runs into the database on reset.

use std::rc::Rc;

use log::{info, warn};

use crate::{db::Result, Db};

/// An observer that logs runs into a database.
pub struct Observer {
    /// A reference to the database.
    db: Rc<Db>,
}

impl crate::model::attempt::Observer for Observer {
    fn on_reset(&self, session: &crate::model::attempt::Session) {
        log_err(self.try_save_run(session))
    }
}

impl Observer {
    #[must_use]
    pub fn new(db: Rc<Db>) -> Self {
        Observer { db }
    }

    fn try_save_run(&self, session: &crate::model::attempt::Session) -> Result<()> {
        if let Some(run) = session.run_as_historic() {
            self.db.add_run(&run)?;
            info!("saved run at {}", run.date);
        } else {
            info!("skipped saving run -- not started")
        }
        Ok(())
    }
}

fn log_err(e: Result<()>) {
    e.unwrap_or_else(|e| warn!("error saving run: {}", e))
}
