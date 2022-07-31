/*! The zombiesplit client, connecting to a zombiesplit server.

This provides services that can then be used by a UI (which we assume to be running on the main
thread in a synchronous context). */

mod error;

use super::{
    super::model::session::{self, action::Handler},
    client::error::Error,
    proto,
};
use error::Result;
use std::sync::Arc;
use tokio::runtime;

/// Lifts a zombiesplit client into a synchronous context.
#[derive(Clone)]
pub struct Sync<O> {
    /// The asynchronous client.
    inner: Client<O>,
    /// The asynchronous runtime.
    rt: Arc<runtime::Runtime>,
}

impl<O: session::Observer> Handler for Sync<O> {
    type Error = error::Error;

    fn dump(&mut self) -> Result<session::State> {
        self.rt.block_on(self.inner.dump())
    }

    fn handle(&mut self, a: session::Action) -> Result<()> {
        self.rt.block_on(self.inner.handle_action(a))
    }
}

impl<O: session::Observer> Sync<O> {
    /// Creates a new client listening to the server at `addr` and observing events with `observer`.
    ///
    /// # Errors
    ///
    /// Fails if we can't create a TCP connection to `addr`.
    pub fn new<A: TryInto<tonic::transport::Uri>>(addr: A, observer: O) -> Result<Self>
    where
        error::Error: From<<A as TryInto<tonic::transport::Uri>>::Error>,
    {
        let rt = Arc::new(
            runtime::Builder::new_current_thread()
                .enable_all()
                .build()?,
        );
        let inner = rt.block_on(Client::new(addr, observer))?;
        Ok(Self { inner, rt })
    }

    /// Runs the observation loop for the client, until the given cancellation channel fires.
    ///
    /// # Errors
    ///
    /// Fails if the client fails to process an action or an event, or there is an underlying I/O
    /// error.
    ///
    /// # Panics
    ///
    /// Can panic if the underlying asynchronous client code panics.
    pub fn observe(&mut self, cancel: tokio::sync::oneshot::Receiver<()>) -> Result<()> {
        self.rt.block_on(self.inner.observe(cancel))?;
        Ok(())
    }
}

impl<O: session::Observer + Clone + Send + 'static> Sync<O> {
    /// Runs `client_code` with this client and an initial dump.
    ///
    /// This also forks a version of this client to pump observations while `client_code` is
    /// running; we assume that the client code is already using the attached `Observer`.
    ///
    /// # Errors
    ///
    /// Fails if the client fails to process an action or an event, or there is an underlying I/O
    /// error, or the client code fails.
    ///
    /// # Panics
    ///
    /// Can panic if the underlying asynchronous client code panics.
    pub fn run<E: From<Error>>(
        &mut self,
        client_code: impl FnOnce(session::State, &mut Self) -> std::result::Result<(), E>,
    ) -> std::result::Result<(), E> {
        let state = self.dump()?;

        let (csend, crecv) = tokio::sync::oneshot::channel();

        // Spawn off a separate cancellable thread to perform the observations.
        let mut observing_client = self.clone();
        let _handle = std::thread::spawn(move || -> Result<()> {
            observing_client.observe(crecv)?;
            Ok(())
        });

        (client_code)(state, self)?;

        csend.send(()).map_err(|_| Error::ObserverCancelFail)?;

        // TODO(@MattWindsor91): make this work

        /*
        handle
            .join()
            .map_err(|e| anyhow::anyhow!("couldn't join client thread"))??;
         */

        Ok(())
    }
}

/// An asynchronous zombiesplit network client.
#[derive(Clone)]
pub struct Client<O> {
    /// The gRPC channel connecting to the server.
    grpc: proto::zombiesplit_client::ZombiesplitClient<tonic::transport::Channel>,
    /// The observer we use to send events from the server.
    observer: O,
}

impl<O: session::Observer> Client<O> {
    /// Creates a new client listening to the server at `addr` and observing events with `observer`.
    ///
    /// # Errors
    ///
    /// Fails if we can't create a TCP connection to `addr`.
    pub async fn new<A>(addr: A, observer: O) -> Result<Self>
    where
        A: TryInto<tonic::transport::Uri>,
        error::Error: From<<A as TryInto<tonic::transport::Uri>>::Error>,
    {
        Ok(Self {
            grpc: proto::zombiesplit_client::ZombiesplitClient::connect(addr.try_into()?).await?,
            observer,
        })
    }

    /// Runs an observer loop for the client.
    ///
    /// The loop will close when the given one-shot is called.
    ///
    /// # Errors
    ///
    /// Fails if the client fails to process an action or an event, or there is an underlying I/O
    /// error.
    ///
    /// # Panics
    ///
    /// Can panic if the underlying select panics.
    pub async fn observe(&mut self, mut cancel: tokio::sync::oneshot::Receiver<()>) -> Result<()> {
        let mut stream = self.event_stream().await?;

        let mut is_running = true;
        while is_running {
            is_running = tokio::select! {
                _ = &mut cancel => false,
                msg = stream.message() => self.handle_event(msg?)?
            }
        }

        Ok(())
    }

    /// Subscribes to an event stream from `gRPC`.
    async fn event_stream(&mut self) -> Result<tonic::codec::Streaming<proto::Event>> {
        Ok(self
            .grpc
            .observe(proto::ObserveRequest {})
            .await?
            .into_inner())
    }

    /// Asks the server to dump the full session state.
    ///
    /// Clients should usually use this once and then subscribe through [observe] to get streaming
    /// updates.
    ///
    /// # Errors
    ///
    /// Fails if any part of the dumping process fails (primarily network or transcoding errors).
    pub async fn dump(&mut self) -> Result<session::State> {
        Ok(proto::decode::dump::dump(&self.dump_raw().await?)?)
    }

    async fn dump_raw(&mut self) -> Result<proto::DumpResponse> {
        Ok(self.grpc.dump(proto::DumpRequest {}).await?.into_inner())
    }

    /// Asks the server to perform an action.
    ///
    /// # Errors
    ///
    /// Fails if any part of the dumping process fails (primarily network or transcoding errors).
    pub async fn handle_action(&mut self, action: session::Action) -> Result<()> {
        match action {
            session::Action::NewRun(dest) => {
                self.grpc
                    .new_attempt(proto::NewAttemptRequest {
                        save: dest == session::action::OldDestination::Save,
                    })
                    .await?;
            }
            session::Action::Push(index, time) => {
                self.grpc
                    .push(proto::encode::action::push(index, time)?)
                    .await?;
            }
            session::Action::Pop(index, ty) => {
                self.grpc
                    .pop(proto::encode::action::pop(index, ty)?)
                    .await?;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event_if_open: Option<proto::Event>) -> Result<bool> {
        if let Some(event) = event_if_open {
            if let Some(e) = proto::decode::event::decode(event)? {
                self.observer.observe(e);
            }
            Ok(true)
        } else {
            log::info!("connection to server closed");
            Ok(false)
        }
    }
}
