/*! The zombiesplit client, connecting to a zombiesplit server.

This provides services that can then be used by a UI (which we assume to be running on the main
thread in a synchronous context). */

pub mod action;
mod error;

use crate::model::attempt;
use error::Result;
use futures::prelude::*;
use tokio::runtime;

/// Lifts a zombiesplit client into a synchronous context.
pub struct Sync<O> {
    /// The asynchronous client.
    inner: Client<O>,
    /// The asynchronous runtime.
    rt: runtime::Runtime,
}

impl<O: attempt::Observer> Sync<O> {
    /// Creates a new client listening to the server at `addr` and observing events with `observer`.
    ///
    /// # Errors
    ///
    /// Fails if we can't create a TCP connection to `addr`.
    pub fn new(
        addr: std::net::SocketAddr,
        observer: O,
        receiver: action::Receiver,
    ) -> Result<Self> {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let inner = rt.block_on(Client::new(addr, observer, receiver))?;
        Ok(Self { inner, rt })
    }

    /// Runs the main loop for the client.
    ///
    /// # Errors
    ///
    /// Fails if the client fails to process an action or an event, or there is an underlying I/O
    /// error.
    ///
    /// # Panics
    ///
    /// Can panic if the underlying asynchronous client code panics.
    pub fn run(&mut self) -> Result<()> {
        self.rt.block_on(self.inner.run())?;
        Ok(())
    }
}

/// A zombiesplit network client.
pub struct Client<O> {
    /// Whether the client is running.
    is_running: bool,
    /// The I/O stack connecting to the server.
    io: super::io::Stack<attempt::observer::Event, attempt::Action>,
    /// The observer we use to send events from the server.
    observer: O,
    /// The receiver we use to pick up actions from the UI code.
    receiver: action::Receiver,
}

impl<O: attempt::Observer> Client<O> {
    /// Creates a new client listening to the server at `addr` and observing events with `observer`.
    ///
    /// # Errors
    ///
    /// Fails if we can't create a TCP connection to `addr`.
    pub async fn new(
        addr: std::net::SocketAddr,
        observer: O,
        receiver: action::Receiver,
    ) -> Result<Self> {
        Ok(Self {
            is_running: true,
            io: super::io::build(tokio::net::TcpStream::connect(addr).await?),
            observer,
            receiver,
        })
    }

    /// Runs the main loop for the client.
    ///
    /// # Errors
    ///
    /// Fails if the client fails to process an action or an event, or there is an underlying I/O
    /// error.
    ///
    /// # Panics
    ///
    /// Can panic if the underlying select panics.
    pub async fn run(&mut self) -> Result<()> {
        while self.is_running {
            tokio::select! {
                msg = self.receiver.0.recv() => self.handle_action(msg).await?,
                msg = self.io.try_next() => self.handle_event(msg?)
            }
        }

        Ok(())
    }

    async fn handle_action(&mut self, maybe_action: Option<attempt::Action>) -> Result<()> {
        if let Some(action) = maybe_action {
            self.io.send(action).await?;
        } else {
            // UI closed its sender channel, so this is the intended end of the client's existence.
            self.is_running = false;
        }
        Ok(())
    }

    fn handle_event(&mut self, maybe_event: Option<attempt::observer::Event>) {
        if let Some(event) = maybe_event {
            self.observer.observe(event);
        } else {
            log::info!("connection to server closed");
            self.is_running = false;
        }
    }
}
