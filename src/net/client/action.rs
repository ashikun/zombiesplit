//! Plumbing for sending actions from a UI to a client.
use crate::model::attempt;
use crate::model::attempt::Action;
use tokio::sync::mpsc;

const BUFFER: usize = 32;

/// Constructs a channel for sending actions from the UI to the client.
#[must_use]
pub fn channel() -> (Sender, Receiver) {
    let (send, recv) = mpsc::channel(BUFFER);
    (Sender(send), Receiver(recv))
}

/// Sender half of an action channel, to send to the presenter.
pub struct Sender(mpsc::Sender<attempt::Action>);

/// Allows the sender to be used in synchronous code as an action handler.
impl attempt::action::Handler for Sender {
    fn handle(&mut self, a: Action) {
        // TODO(@MattWindsor91): deal with errors properly
        if let Err(e) = self.0.blocking_send(a) {
            log::error!("error sending action from presenter: {}", e);
        }
    }
}

/// Receiver half of an action channel, to send to the client.
pub struct Receiver(pub(super) mpsc::Receiver<attempt::Action>);
