/*! The zombiesplit server.

This is a TCP server that hosts a run attempt session, accepts actions to perform on that attempt,
and emits observations that reflect changes to the attempt.
*/

use std::sync::{Arc, Weak};

use tokio::sync::{broadcast, mpsc, oneshot};

pub use error::{Error, Result};

use crate::model::{
    attempt::{observer::Event, sink},
    timing::comparison::provider,
};

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
        game::category::ShortDescriptor,
    },
};

mod error;
mod grpc;
mod listener;

/// A manager of a zombiesplit server.
///
/// This holds the configuration and database handles that will be used by the server proper.
pub struct Manager {
    cfg: config::Server,

    //
    // Database
    //

    // Reader for acquiring comparisons.
    reader: db::Reader,
    // Sink for completed runs.
    sink: db::Sink,

    //
    // Message routing
    //
    /// Send/receive pair for broadcasting events from the session to clients.
    /// We hold the receiver here to keep it alive.
    bcast: (
        broadcast::Sender<attempt::observer::Event>,
        broadcast::Receiver<attempt::observer::Event>,
    ),

    //
    // Observers
    //
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

impl Manager {
    /// Constructs a new server, opening a database connection.
    ///
    /// # Errors
    ///
    /// Returns any errors from trying to open the database.
    pub fn new(cfg: config::Server) -> Result<Self> {
        let db = std::rc::Rc::new(db::Db::new(&cfg.db.path)?);
        let reader = db.reader()?;

        let debug_obs: Arc<dyn attempt::Observer> = Arc::new(Debug);

        let bcast = tokio::sync::broadcast::channel(BCAST_CAPACITY);
        let bcast_obs: Arc<dyn attempt::Observer> = Arc::new(Broadcast(bcast.0.clone()));

        let mut m = Self {
            cfg,
            reader,
            bcast,
            sink: db::Sink::new(db),
            observers: vec![debug_obs, bcast_obs],
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
        let (message_send, message_recv) = tokio::sync::mpsc::channel(MPSC_CAPACITY);
        Ok(Server {
            addr: self.cfg.net.address,
            handler: grpc::Handler {
                message_send,
                event_broadcast: self.bcast.0.clone(),
            },
            state: State {
                session: self.session(insp)?,
                message_recv,
            },
        })
    }

    fn session<'a, 'db>(
        &'a self,
        mut insp: Inspector<'db>,
    ) -> Result<attempt::Session<'db, 'a, model::attempt::observer::Mux>> {
        let mut session = insp.init_session(&self.obs_mux)?;
        session.set_comparison_provider(self.comparison_provider(insp));
        session.set_sink(self.sink());
        Ok(session)
    }

    fn comparison_provider<'a>(&self, insp: Inspector<'a>) -> Box<dyn provider::Provider + 'a> {
        match self.cfg.comparison.provider {
            config::server::comparison::Provider::Database => Box::new(insp),
            _ => Box::new(provider::Null),
        }
    }

    fn sink(&self) -> Box<dyn sink::Sink> {
        // TODO(@MattWindsor91): let users turn off database sinking
        Box::new(self.sink.clone())
    }
}

/// Observers can be attached to the manager, for use in the session later.
impl Observable for Manager {
    fn add_observer(&mut self, observer: Weak<dyn Observer>) {
        self.obs_mux.add_observer(observer);
    }
}

/// A server, wrapping a session with the means to control it (through gRPC).
///
/// A server owns a running session, as well as the various observers attached to it, and performs
/// many of the tasks of bringing up, maintaining, and tearing down those elements.
///
/// In future, a server will also expose its control plane to clients.
///
/// The lifetime `m` generally reflects that of its underlying `Manager`.
pub struct Server<'m> {
    addr: std::net::SocketAddr,
    handler: grpc::Handler,
    state: State<'m>,
}

impl<'cmp> Server<'cmp> {
    /// Runs the server, consuming it.
    pub async fn run(self) {
        tokio::spawn(run_grpc(self.addr, self.handler));

        let mut state = self.state;
        state.run().await;
    }
}

async fn run_grpc(addr: std::net::SocketAddr, handler: grpc::Handler) {
    let srv = super::proto::zombiesplit_server::ZombiesplitServer::new(handler);
    if let Err(e) = tonic::transport::server::Server::builder()
        .concurrency_limit_per_connection(256)
        .add_service(srv)
        .serve(addr)
        .await
    {
        // TODO(@MattWindsor91): handle error properly
        log::error!("error in server: {e}");
    }
}

/// The state part of the server.
struct State<'m> {
    /// The session being wrapped by this server.
    session: attempt::Session<'m, 'm, attempt::observer::Mux>,
    /// Receives messages from the server handler.
    message_recv: mpsc::Receiver<Message>,
}

/// A message to the server.
#[derive(Debug)]
pub enum Message {
    /// An action to send to the session; no direct reply expected.
    Action(attempt::Action),
    /// A dumping query, which expects a reply through the given oneshot.
    Dump(oneshot::Sender<super::dump::Dump>),
}

impl<'m> State<'m> {
    /// Runs the state main loop, which constantly drains messages from clients and applies them.
    ///
    /// These messages, in turn, give rise to observations that will bubble up through the broadcast
    /// channel and into clients.
    async fn run(&mut self) {
        while let Some(msg) = self.message_recv.recv().await {
            match msg {
                Message::Action(act) => self.session.handle(act),
                Message::Dump(rx) => {
                    // TODO(@MattWindsor91): handle drop?
                    let _res = rx.send(self.dump());
                }
            }
        }
    }

    fn dump(&self) -> super::dump::Dump {
        super::dump::Dump {
            server: super::dump::Server {
                ident: SERVER_IDENT.to_string(),
                version: SERVER_VERSION,
            },
            run: self.session.dump(),
        }
    }
}

const SERVER_IDENT: &str = "zsserver";
const SERVER_VERSION: semver::Version = semver::Version::new(0, 1, 0);

// TODO(@MattWindsor91): https://github.com/MattWindsor91/zombiesplit/issues/23

/// Number of events for which we reserve space in the broadcast channel.
const BCAST_CAPACITY: usize = 100;

/// Number of actions for which we reserve space in the MPSC channel.
const MPSC_CAPACITY: usize = 16;
