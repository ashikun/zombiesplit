//! TCP listening logic for the zombiesplit server.
use super::Result;
use crate::model::attempt;
use futures::prelude::*;
use tokio::{
    net::TcpListener,
    sync::{broadcast, mpsc},
};

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
            log::info!("new connection: {addr}");

            // Deserialize frames
            let mut handle = Handle {
                addr,
                io: super::super::io::build(socket),
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
    io: super::super::io::Stack<attempt::Action, attempt::observer::Event>,

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
            log::warn!("connection closed with error: {} ({e:?})", self.addr);
        }
    }

    async fn main_loop(&mut self) -> Result<()> {
        while self.is_running {
            tokio::select! {
                msg = self.event_recv.recv() => self.handle_event(msg).await?,
                msg = self.io.try_next() => self.handle_action(msg?).await?
            }
        }
        Ok(())
    }

    /// Handles a potential action from the client to the session.
    async fn handle_action(&mut self, maybe_action: Option<attempt::Action>) -> Result<()> {
        if let Some(action) = maybe_action {
            self.action_send.send(action).await?;
        } else {
            log::info!("connection closed: {}", self.addr);
            self.is_running = false;
        }

        Ok(())
    }

    /// Handles a potential event from the session to the client.
    async fn handle_event(
        &mut self,
        maybe_event: std::result::Result<attempt::observer::Event, broadcast::error::RecvError>,
    ) -> Result<()> {
        if let Err(broadcast::error::RecvError::Lagged(n)) = maybe_event {
            log::info!("connection lagging: {} ({n} behind)", self.addr);
            // TODO(@MattWindsor91): deal with this properly
        } else {
            self.io.send(maybe_event?).await?;
        }
        Ok(())
    }
}
