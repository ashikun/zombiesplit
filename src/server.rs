/*! The zombiesplit server.

This is a TCP server that hosts a run attempt session, accepts actions to perform on that attempt,
and emits observations that reflect changes to the attempt.
*/

mod router;

use futures::prelude::*;
use std::sync::{Arc, Weak};

use super::{
    config,
    db::{self, inspect::Inspector},
    model::{
        self,
        attempt::{
            self,
            action::Handler,
            observer::{Debug, Observable, Observer},
            Action,
        },
        comparison,
        game::category::ShortDescriptor,
    },
    ui,
};
use thiserror::Error;
use tokio::{net::TcpListener, sync::mpsc};
use tokio_serde::formats::Cbor;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// A manager of a zombiesplit server.
///
/// This holds the configuration and database handles that will be used by the server proper.
pub struct Manager<'c> {
    cfg: config::System<'c>,
    reader: db::Reader,

    db_obs: Arc<dyn attempt::Observer>,
    debug_obs: Arc<dyn attempt::Observer>,
    obs_mux: attempt::observer::Mux,
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

        let mut m = Self {
            cfg,
            reader,
            db_obs,
            debug_obs,
            obs_mux: attempt::observer::Mux::default(),
        };
        m.obs_mux.add_observer(Arc::downgrade(&m.db_obs));
        m.obs_mux.add_observer(Arc::downgrade(&m.debug_obs));

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
            session: self.session(insp)?,
            addr: self.cfg.server_addr,
            action_in,
            action_out,
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
    // TODO(@MattWindsor91): de-public all of these
    /// The session being wrapped by this server.
    session: attempt::Session<'m, 'm, attempt::observer::Mux>,

    addr: std::net::SocketAddr,

    action_in: mpsc::Sender<attempt::Action>,
    action_out: mpsc::Receiver<attempt::Action>,
}

impl<'cmp> Server<'cmp> {
    /// Runs the server.
    ///
    /// # Errors
    ///
    /// Propagates various I/O errors from tokio.
    ///
    /// # Panics
    ///
    /// Temporary panicking, to be fixed later.
    pub async fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            tokio::select! {
                Some(act) = self.action_out.recv() => {
                    self.session.handle(act)
                },
                res = async {
                    loop {
                        let (socket, _) = listener.accept().await?;

                        // Delimit frames using a length header
                        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

                        let codec: Cbor<attempt::Action, attempt::observer::Event> = Cbor::default();

                        // Deserialize frames
                        let deserialized: tokio_serde::Framed<
                            _,
                            attempt::Action,
                            attempt::observer::Event,
                            _,
                        > = tokio_serde::Framed::new(length_delimited, codec);

                        let (send, mut recv) = deserialized.split();

                        // Spawn a task that prints all received messages to STDOUT
                        tokio::spawn(async move {
                            while let Some(msg) = recv.try_next().await.unwrap() {
                                println!("GOT: {:?}", msg);
                            }
                        });
                    }

                    // Help the rust type inferencer out
                    Ok::<_, std::io::Error>(())
                } => {res?}
            }
        }
    }

    /// Creates an action handler that can be used to send actions to a running server.
    #[must_use]
    pub fn handler(&self) -> ActionForwarder {
        ActionForwarder(self.action_in.clone())
    }
}

/// An action handler that forwards actions directly to the session.
pub struct ActionForwarder(mpsc::Sender<attempt::Action>);

impl attempt::action::Handler for ActionForwarder {
    fn handle(&mut self, a: Action) {
        // TODO(@MattWindsor91): errors
        if let Err(err) = self.0.blocking_send(a) {
            log::error!("can't forward action: {}", err)
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
    #[error("IO error")]
    IO(#[from] std::io::Error),
}

/// The top-level server result type.
pub type Result<T> = std::result::Result<T, Error>;
