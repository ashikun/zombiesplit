/*! The zombiesplit server.

This is a TCP server that hosts a run attempt session, accepts actions to perform on that attempt,
and emits observations that reflect changes to the attempt.
*/

mod router;

use futures::prelude::*;
use std::rc::{Rc, Weak};

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
use crate::model::attempt::observer::Observable;
use crate::model::attempt::Observer;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio_serde::formats::Cbor;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// A manager of a zombiesplit server.
///
/// This holds the configuration and database handles that will be used by the server proper.
pub struct Manager<'c> {
    cfg: config::System<'c>,
    reader: db::Reader,

    db_obs: Rc<dyn attempt::Observer>,
    debug_obs: Rc<dyn attempt::Observer>,
    obs_mux: attempt::observer::Mux,
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

        let db_obs: Rc<dyn attempt::Observer> = Rc::new(db::Observer::new(db));
        let debug_obs: Rc<dyn attempt::Observer> = Rc::new(Debug);

        let mut m = Self {
            cfg,
            reader,
            db_obs,
            debug_obs,
            obs_mux: attempt::observer::Mux::default(),
        };
        m.obs_mux.add_observer(Rc::downgrade(&m.db_obs));
        m.obs_mux.add_observer(Rc::downgrade(&m.debug_obs));

        Ok(m)
    }

    /// Creates a server for the given game.
    ///
    /// # Errors
    ///
    /// Returns any database or UI errors caught during the session.
    pub fn server(&self, desc: &ShortDescriptor) -> Result<Server> {
        let insp = self.reader.inspect(desc)?;
        Ok(Server {
            session: self.session(insp)?,
            addr: self.cfg.server_addr,
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
    pub session: attempt::Session<'m, 'm, attempt::observer::Mux>,

    pub addr: std::net::SocketAddr,
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
            let (socket, _) = listener.accept().await.unwrap();

            // Delimit frames using a length header
            let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

            let codec: Cbor<attempt::Action, attempt::observer::Event> = Cbor::default();

            // Deserialize frames
            let mut deserialized: tokio_serde::Framed<
                _,
                attempt::Action,
                attempt::observer::Event,
                _,
            > = tokio_serde::Framed::new(length_delimited, codec);

            // Spawn a task that prints all received messages to STDOUT
            tokio::spawn(async move {
                while let Some(msg) = deserialized.try_next().await.unwrap() {
                    println!("GOT: {:?}", msg);
                }
            });
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
