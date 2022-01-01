/*! The zombiesplit server.

This is a TCP server that hosts a run attempt session, accepts actions to perform on that attempt,
and emits observations that reflect changes to the attempt.
*/

mod error;
mod listener;

use std::sync::{Arc, Weak};

use super::super::{
    config,
    db::{self, inspect::Inspector},
    model::{
        self,
        attempt::{
            self,
            action::Handler,
            observer::{Debug, Observable, Observer},
        },
        comparison,
        game::category::ShortDescriptor,
    },
};
use crate::model::attempt::observer::Event;
use tokio::sync::{broadcast, mpsc};

pub use error::{Error, Result};

/// A manager of a zombiesplit server.
///
/// This holds the configuration and database handles that will be used by the server proper.
pub struct Manager<'c> {
    cfg: config::System<'c>,
    reader: db::Reader,

    /// Send/receive pair for broadcasting events from the session to clients.
    /// We hold the receiver here to keep it alive.
    bcast: (
        broadcast::Sender<attempt::observer::Event>,
        broadcast::Receiver<attempt::observer::Event>,
    ),

    observers: Vec<Arc<dyn attempt::Observer>>,
    obs_mux: attempt::observer::Mux,
}

struct Broadcast(tokio::sync::broadcast::Sender<attempt::observer::Event>);
impl attempt::Observer for Broadcast {
    fn observe(&self, evt: Event) {
        if let Err(e) = self.0.send(evt) {
            log::error!("couldn't send observation to clients: {}", e);
        }
    }
}

impl<'c> Manager<'c> {
    /// Constructs a new server, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::System<'c>) -> Result<Self> {
        let db = std::rc::Rc::new(db::Db::new(&cfg.db_path)?);
        let reader = db.reader()?;

        let db_obs: Arc<dyn attempt::Observer> = Arc::new(db::Observer::new(db));
        let debug_obs: Arc<dyn attempt::Observer> = Arc::new(Debug);

        let bcast = tokio::sync::broadcast::channel(32);
        let bcast_obs: Arc<dyn attempt::Observer> = Arc::new(Broadcast(bcast.0.clone()));

        let mut m = Self {
            cfg,
            reader,
            bcast,
            observers: vec![db_obs, debug_obs, bcast_obs],
            obs_mux: attempt::observer::Mux::default(),
        };

        for obs in &m.observers {
            m.obs_mux.add_observer(Arc::downgrade(obs));
        }

        Ok(m)
    }

    /// Creates a server for the given game.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn server(&self, desc: &ShortDescriptor) -> Result<Server> {
        let insp = self.reader.inspect(desc)?;
        let (action_in, action_out) = tokio::sync::mpsc::channel(32);
        Ok(Server {
            listener: listener::Listener::new(
                self.cfg.server_addr,
                action_in,
                self.bcast.0.clone(),
            ),
            state: State {
                session: self.session(insp)?,
                action_out,
            },
        })
    }

    fn session<'a, 'db>(
        &'a self,
        mut insp: Inspector<'db>,
    ) -> Result<model::attempt::Session<'db, 'a, model::attempt::observer::Mux>> {
        let mut session = insp.init_session(&self.obs_mux)?;
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

/// Observers can be attached to the manager, for use in the session later.
impl<'c> Observable for Manager<'c> {
    fn add_observer(&mut self, observer: Weak<dyn Observer>) {
        self.obs_mux.add_observer(observer);
    }
}

/// A server, wrapping a session with the means to control it.
///
/// A server owns a running session, as well as the various observers attached to it, and performs
/// many of the tasks of bringing up, maintaining, and tearing down those elements.
///
/// In future, a server will also expose its control plane to clients.
///
/// The lifetime `m` generally reflects that of its underlying `Manager`.
pub struct Server<'m> {
    listener: listener::Listener,
    state: State<'m>,
}

impl<'cmp> Server<'cmp> {
    /// Runs the server, consuming it.
    pub async fn run(self) {
        let mut listener = self.listener;
        let mut state = self.state;
        tokio::spawn(async move { listener.run().await });

        state.run().await;
    }
}

/// The state part of the server.
struct State<'m> {
    /// The session being wrapped by this server.
    session: attempt::Session<'m, 'm, attempt::observer::Mux>,
    action_out: mpsc::Receiver<attempt::Action>,
}

impl<'m> State<'m> {
    /// Runs the state main loop, which constantly drains actions from clients and applies them.
    ///
    /// These actions, in turn, give rise to observations that will bubble up through the broadcast
    /// channel and into clients.
    async fn run(&mut self) {
        while let Some(act) = self.action_out.recv().await {
            self.session.handle(act);
        }
    }
}
