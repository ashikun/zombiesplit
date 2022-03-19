/*! Logic for observing session events using a presenter.

Generally, one would not directly connect a session to a presenter, as the two will reside on
separate sides of a client/server setup.  However, we still architect the presenter as if this were
the case, both for historical and future-proofing reasons.

The observer has two parts: the observer proper, and a pump that drains observations sent to the
observer to feed to the session. */

use super::{super::super::model::session, Presenter};
use std::sync::mpsc;

/// Used to feed events from an `Observer` into a `Presenter`.
pub struct Pump(mpsc::Receiver<session::event::Event>);

/// Creates an observer as well as a pump that feeds events from the observer into a presenter.
#[must_use]
pub fn make() -> (Observer, Pump) {
    let (send, recv) = mpsc::channel();
    (Observer(send), Pump(recv))
}

impl Pump {
    /// Pumps this event forwarder's event queue, pushing each event to `to`.
    pub fn pump(&mut self, to: &mut Presenter<impl session::action::Handler>) {
        self.0.try_iter().for_each(|x| to.observe(&x));
    }
}

/// An observer that feeds into a [Presenter].
#[derive(Clone)]
pub struct Observer(mpsc::Sender<session::event::Event>);

impl session::Observer for Observer {
    fn observe(&self, evt: session::event::Event) {
        // TODO(@MattWindsor91): handle errors properly?
        if let Err(e) = self.0.send(evt) {
            log::warn!("error sending event to presenter: {e}");
        }
    }
}
