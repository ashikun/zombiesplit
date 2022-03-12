//! `gRPC` glue for the server.

use super::super::{
    super::model::session,
    proto::{self, zombiesplit_server::Zombiesplit},
};
use crate::net::proto::{decode, encode};
use futures::StreamExt;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc, oneshot};

type EventStream =
    Pin<Box<dyn futures::Stream<Item = std::result::Result<proto::Event, tonic::Status>> + Send>>;

/// `gRPC` handler for the zombiesplit server.
#[derive(Clone)]
pub struct Handler {
    /// The main sender channel for actions (pointing back towards the server).
    pub message_send: mpsc::Sender<super::Message>,
    /// A broadcast channel head for events, from which we subscribe new event receivers.
    pub event_broadcast: broadcast::Sender<session::event::Event>,
}

type Result<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl Zombiesplit for Handler {
    type ObserveStream = EventStream;

    async fn server_info(
        &self,
        _request: tonic::Request<proto::ServerInfoRequest>,
    ) -> Result<proto::ServerInfoResponse> {
        self.query(
            "server info",
            super::Message::ServerInfo,
            proto::encode::server_info,
        )
        .await
    }

    async fn dump(
        &self,
        _request: tonic::Request<proto::DumpRequest>,
    ) -> Result<proto::DumpResponse> {
        self.query("dump", super::Message::Dump, encode::dump::encode)
            .await
    }

    async fn new_attempt(
        &self,
        request: tonic::Request<proto::NewAttemptRequest>,
    ) -> Result<proto::NewAttemptResponse> {
        let dest = if request.into_inner().save {
            session::action::OldDestination::Save
        } else {
            session::action::OldDestination::Discard
        };

        self.act(session::Action::NewRun(dest)).await?;
        // TODO(@MattWindsor91): report back whether the save occurred.
        Ok(tonic::Response::new(proto::NewAttemptResponse {}))
    }

    async fn push(
        &self,
        request: tonic::Request<proto::PushRequest>,
    ) -> Result<proto::PushResponse> {
        self.act(decode::action::push(&request.into_inner())?)
            .await?;
        Ok(tonic::Response::new(proto::PushResponse {}))
    }

    async fn pop(&self, request: tonic::Request<proto::PopRequest>) -> Result<proto::PopResponse> {
        self.act(decode::action::pop(&request.into_inner())?)
            .await?;
        Ok(tonic::Response::new(proto::PopResponse {}))
    }

    async fn observe(
        &self,
        _request: tonic::Request<proto::ObserveRequest>,
    ) -> Result<Self::ObserveStream> {
        let recv = self.event_broadcast.subscribe();
        let recv_stream = tokio_stream::wrappers::BroadcastStream::new(recv);
        let mapped_stream = recv_stream.map(|x| map_event_result(&x));
        let response = Pin::new(Box::new(mapped_stream));
        Ok(tonic::Response::new(response))
    }
}

impl Handler {
    /// Sends `action` asynchronously to the session.
    ///
    /// # Errors
    ///
    /// Fails if the underlying send fails.
    async fn act(&self, action: session::Action) -> std::result::Result<(), tonic::Status> {
        self.message_send
            .send(super::Message::Action(action))
            .await
            .map_err(|x| tonic::Status::internal(x.to_string()))
    }

    /// Handles the main body of a RPC call that just asks the server to provide some information.
    async fn query<T, P>(
        &self,
        type_name: &'static str,
        to_message: fn(oneshot::Sender<T>) -> super::Message,
        encoder: fn(&T) -> proto::encode::Result<P>,
    ) -> Result<P> {
        let (send, recv) = oneshot::channel();

        self.message_send
            .send(to_message(send))
            .await
            .map_err(|e| {
                tonic::Status::internal(format!("couldn't send {type_name} message: {e}"))
            })?;

        let reply = recv
            .await
            .map_err(|_| tonic::Status::internal(format!("{type_name} channel dropped")))?;

        encoder(&reply).map(tonic::Response::new)
    }
}

fn map_event_result(
    event: &std::result::Result<
        session::event::Event,
        tokio_stream::wrappers::errors::BroadcastStreamRecvError,
    >,
) -> std::result::Result<proto::Event, tonic::Status> {
    event
        .as_ref()
        .map_err(map_event_error)
        .and_then(proto::encode::event::encode)
}

fn map_event_error(
    err: &tokio_stream::wrappers::errors::BroadcastStreamRecvError,
) -> tonic::Status {
    match err {
        tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(e) => {
            tonic::Status::data_loss(format!("lagged behind: {e}"))
        }
    }
}
