//! TCP listening logic for the zombiesplit server.
use super::Result;
use crate::model::attempt;
use futures::prelude::*;
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};
use tokio_serde::formats::Cbor;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// A TCP listener, spawning tasks to handle connections to clients.
pub struct Listener {
    /// The address on which this listener is listening.
    addr: std::net::SocketAddr,
    /// The main sender channel for actions (pointing back towards the server).
    action_send: mpsc::Sender<attempt::Action>,
    /// A broadcast channel head for events, from which we subscribe new event receivers.
    event_broadcast: broadcast::Sender<attempt::observer::Event>,
}

impl Listener {
    pub fn new(
        addr: std::net::SocketAddr,
        action_send: mpsc::Sender<attempt::Action>,
        event_broadcast: broadcast::Sender<attempt::observer::Event>,
    ) -> Self {
        Self {
            addr,
            action_send,
            event_broadcast,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (socket, addr) = listener.accept().await?;
            log::info!("new connection: {}", addr);

            // Delimit frames using a length header
            let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

            let codec: Cbor<attempt::Action, attempt::observer::Event> = Cbor::default();

            // Deserialize frames
            let mut handle = Handle {
                addr,
                io: tokio_serde::Framed::new(length_delimited, codec),
                action_send: self.action_send.clone(),
                event_recv: self.event_broadcast.subscribe(),
                is_running: true,
            };

            handle.action_send.send(attempt::Action::Dump).await?;

            tokio::spawn(async move { handle.run().await });
        }
    }
}

/// State for tracking one client connection.
struct Handle {
    /// The address of the peer.
    addr: std::net::SocketAddr,

    /// The I/O stack connecting to the client.
    io: tokio_serde::Framed<
        tokio_util::codec::Framed<tokio::net::TcpStream, tokio_util::codec::LengthDelimitedCodec>,
        attempt::Action,
        attempt::observer::Event,
        tokio_serde::formats::Cbor<attempt::Action, attempt::observer::Event>,
    >,

    /// Sends actions to the session.
    action_send: mpsc::Sender<attempt::Action>,

    /// Receives events from the session.
    event_recv: broadcast::Receiver<attempt::observer::Event>,

    /// Whether the handle is running.
    is_running: bool,
}

impl Handle {
    async fn run(&mut self) {
        if let Err(e) = self.main_loop().await {
            log::warn!("connection closed with error: {} ({:?})", self.addr, e)
        }
    }

    async fn main_loop(&mut self) -> Result<()> {
        while self.is_running {
            tokio::select! {
                msg = self.event_recv.recv() => self.handle_incoming(msg).await?,
                from_client = self.io.try_next() => {
                    if let Some(action) = from_client? {
                        self.action_send.send(action).await?;
                    } else {
                        break
                    }
                }
            }
        }
        Ok(())
    }

    /// Handles a potential incoming message from the client.
    /// Returns whether the client has closed.
    async fn handle_incoming(
        &mut self,
        maybe_event: std::result::Result<attempt::observer::Event, broadcast::error::RecvError>,
    ) -> Result<()> {
        match maybe_event {
            // TODO(@MattWindsor91): handle errors properly?
            Ok(event) => {
                self.io.send(event).await?;
            }
            Err(broadcast::error::RecvError::Closed) => {
                log::info!("connection closed: {}", self.addr);
                self.is_running = false;
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                log::info!("connection lagging: {} ({} behind)", self.addr, n);
                // TODO(@MattWindsor91): deal with this properly
            }
        }
        Ok(())
    }
}
