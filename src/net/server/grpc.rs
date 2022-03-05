//! `gRPC` glue for the server.

use super::super::{
    super::model::attempt,
    proto::{self, zombiesplit_server::Zombiesplit},
};
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
    pub event_broadcast: broadcast::Sender<attempt::observer::Event>,
}

type Result<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl Zombiesplit for Handler {
    type ObserveStream = EventStream;

    async fn dump(
        &self,
        _request: tonic::Request<proto::DumpRequest>,
    ) -> Result<proto::DumpResponse> {
        let (send, recv) = oneshot::channel();

        self.message_send
            .send(super::Message::Dump(send))
            .await
            .map_err(|e| tonic::Status::internal(format!("couldn't send dump message: {e}")))?;

        let dump = recv
            .await
            .map_err(|_| tonic::Status::internal("dump channel dropped"))?;

        Ok(tonic::Response::new(proto::encode::dump(&dump)?))
    }

    async fn new_run(
        &self,
        _request: tonic::Request<proto::NewRunRequest>,
    ) -> Result<proto::NewRunResponse> {
        self.act(attempt::Action::NewRun).await?;
        Ok(tonic::Response::new(proto::NewRunResponse {}))
    }

    async fn modify_split(
        &self,
        request: tonic::Request<proto::ModifySplitRequest>,
    ) -> Result<proto::ModifySplitResponse> {
        if let Some(a) = modify_split_action(request.get_ref())? {
            self.act(a).await?;
        }
        Ok(tonic::Response::new(proto::ModifySplitResponse {}))
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

fn map_event_result(
    event: &std::result::Result<
        attempt::observer::Event,
        tokio_stream::wrappers::errors::BroadcastStreamRecvError,
    >,
) -> std::result::Result<proto::Event, tonic::Status> {
    event
        .as_ref()
        .map_err(map_event_error)
        .and_then(proto::encode::event)
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

impl Handler {
    /// Sends `action` asynchronously to the session.
    ///
    /// # Errors
    ///
    /// Fails if the underlying send fails.
    async fn act(&self, action: attempt::Action) -> std::result::Result<(), tonic::Status> {
        self.message_send
            .send(super::Message::Action(action))
            .await
            .map_err(|x| tonic::Status::internal(x.to_string()))
    }
}

fn modify_split_action(
    message: &proto::ModifySplitRequest,
) -> std::result::Result<Option<attempt::Action>, tonic::Status> {
    let index = proto::decode::split_index(message.index)?;
    message
        .modification
        .as_ref()
        .map(|modi| match modi {
            proto::modify_split_request::Modification::Push(stamp) => push(index, *stamp),
            proto::modify_split_request::Modification::Pop(ty) => pop(index, *ty),
        })
        .transpose()
}

fn push(index: usize, stamp: u32) -> std::result::Result<attempt::Action, tonic::Status> {
    Ok(attempt::Action::Push(index, proto::decode::time(stamp)?))
}

fn pop(index: usize, ty: i32) -> std::result::Result<attempt::Action, tonic::Status> {
    match proto::modify_split_request::Pop::from_i32(ty) {
        Some(proto::modify_split_request::Pop::One) => Ok(attempt::Action::Pop(index)),
        Some(proto::modify_split_request::Pop::All) => Ok(attempt::Action::Clear(index)),
        None => Err(tonic::Status::out_of_range(format!("bad pop type: {ty}"))),
    }
}
