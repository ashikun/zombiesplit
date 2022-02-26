//! gRPC glue for the server.

use super::super::{
    super::model::{
        attempt::{self, observer},
        timing::time,
        Time,
    },
    proto::{
        self, event, zombiesplit_server::Zombiesplit, DumpRequest, DumpResponse, Event,
        ModifySplitRequest, ModifySplitResponse, NewRunRequest, NewRunResponse, ObserveRequest,
    },
};
use futures::StreamExt;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc, oneshot};

type EventStream =
    Pin<Box<dyn futures::Stream<Item = std::result::Result<proto::Event, tonic::Status>> + Send>>;

/// gRPC handler for the zombiesplit server.
pub struct Handler {
    /// The main sender channel for actions (pointing back towards the server).
    message_send: mpsc::Sender<super::Message>,
    /// A broadcast channel head for events, from which we subscribe new event receivers.
    event_broadcast: broadcast::Sender<attempt::observer::Event>,
}

type Result<T> = std::result::Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl Zombiesplit for Handler {
    type ObserveStream = EventStream;

    async fn dump(&self, _request: tonic::Request<DumpRequest>) -> Result<DumpResponse> {
        let (send, recv) = oneshot::channel();

        self.message_send
            .send(super::Message::Dump(send))
            .await
            .map_err(|e| tonic::Status::internal(format!("couldn't send dump message: {e}")))?;

        let dump = recv
            .await
            .map_err(|_| tonic::Status::internal("dump channel dropped"))?;

        // Ignored for now.
        Ok(tonic::Response::new(map_dump(&dump)))
    }

    async fn new_run(&self, _request: tonic::Request<NewRunRequest>) -> Result<NewRunResponse> {
        self.act(attempt::Action::NewRun).await?;
        Ok(tonic::Response::new(NewRunResponse {}))
    }

    async fn modify_split(
        &self,
        request: tonic::Request<ModifySplitRequest>,
    ) -> Result<ModifySplitResponse> {
        if let Some(a) = modify_split_action(request.get_ref())? {
            self.act(a).await?;
        }
        Ok(tonic::Response::new(ModifySplitResponse {}))
    }

    async fn observe(
        &self,
        _request: tonic::Request<ObserveRequest>,
    ) -> Result<Self::ObserveStream> {
        let recv = self.event_broadcast.subscribe();
        let recv_stream = tokio_stream::wrappers::BroadcastStream::new(recv);
        let mapped_stream = recv_stream.map(map_event_result);
        let response = Pin::new(Box::new(mapped_stream));
        Ok(tonic::Response::new(response))
    }
}

fn map_dump(_dump: &super::Dump) -> proto::DumpResponse {
    // for now
    proto::DumpResponse::default()
}

fn map_event_result(
    event: std::result::Result<
        attempt::observer::Event,
        tokio_stream::wrappers::errors::BroadcastStreamRecvError,
    >,
) -> std::result::Result<proto::Event, tonic::Status> {
    event.map(map_event).map_err(map_event_error)
}

fn map_event(event: attempt::observer::Event) -> proto::Event {
    Event {
        payload: match event {
            observer::Event::Total(_, _) => None,
            observer::Event::SumOfBest(_) => None,
            observer::Event::NumSplits(_) => None,
            observer::Event::Reset => Some(event::Payload::Control(event::Control::Reset as i32)),
            observer::Event::Attempt(_) => None,
            observer::Event::GameCategory(_) => None,
            observer::Event::Split(_, _) => None,
        },
    }
}

fn map_event_error(err: tokio_stream::wrappers::errors::BroadcastStreamRecvError) -> tonic::Status {
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
    message: &ModifySplitRequest,
) -> std::result::Result<Option<attempt::Action>, tonic::Status> {
    let index = read_index(message.index)?;
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
    Ok(attempt::Action::Push(index, read_time(stamp)?))
}

fn pop(index: usize, ty: i32) -> std::result::Result<attempt::Action, tonic::Status> {
    match proto::modify_split_request::Pop::from_i32(ty) {
        Some(proto::modify_split_request::Pop::One) => Ok(attempt::Action::Pop(index)),
        Some(proto::modify_split_request::Pop::All) => Ok(attempt::Action::Clear(index)),
        None => Err(tonic::Status::out_of_range(format!("bad pop type: {ty}"))),
    }
}

/// Converts a gRPC split index to one used in the model.
///
/// # Errors
///
/// Fails with `out_of_range` if the index is too large (which may happen on eg. 32-bit systems).
fn read_index(index: u64) -> std::result::Result<usize, tonic::Status> {
    usize::try_from(index).map_err(|e| tonic::Status::out_of_range(e.to_string()))
}

fn read_time(stamp: u32) -> std::result::Result<Time, tonic::Status> {
    Time::try_from(stamp).map_err(adapt_time_error)
}

fn adapt_time_error(err: time::Error) -> tonic::Status {
    match err {
        time::Error::MsecOverflow(k) => {
            tonic::Status::out_of_range(format!("millisecond value {k} too large"))
        }
        _ => tonic::Status::invalid_argument(format!("invalid time: {err}")),
    }
}
