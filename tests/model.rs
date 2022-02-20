//! Model integration tests.
//!
//! This also includes UI integration tests that depend on the model, for now.

use std::{cell::RefCell, rc::Rc};

use pretty_assertions::{assert_eq, assert_ne};

use zombiesplit::{
    model::{
        attempt::{
            action::{Action, Handler},
            observer::{self, Event},
            Observer, Run, Session,
        },
        game::{category, Split},
        timing::aggregate,
    },
    ui::{presenter, Pump},
};

/// Tests that a session doesn't send any observations until prompted.
#[test]
fn test_session_initial() {
    let obs = Obs::default();
    let _ = make_session(&obs);
    obs.assert_empty()
}

/// Tests that a session sends the appropriate initial observations when asked
/// to dump.
#[test]
fn test_session_dump() {
    let obs = Obs::default();
    let mut s = make_session(&obs);

    s.handle(Action::Dump);

    obs.assert_received(Event::GameCategory(game_category()));

    let splits = splits();

    obs.assert_received(Event::NumSplits(splits.len()));
    for (index, split) in splits.iter().enumerate() {
        obs.assert_received(Event::Split(
            split.short,
            observer::split::Event::Init {
                index,
                name: split.name.clone(),
            },
        ));
    }
    obs.assert_received(Event::Attempt(attempt_info()));
    obs.assert_received(Event::Total(
        Default::default(),
        aggregate::Source::Comparison,
    ));
    obs.assert_received(Event::SumOfBest(Default::default()));
    obs.assert_empty()
}

fn attempt_info() -> category::AttemptInfo {
    category::AttemptInfo {
        total: 42,
        completed: 2,
    }
}

fn game_category() -> category::Info {
    category::Info {
        game: "Game Name".to_string(),
        category: "Game Category".to_string(),
        short: category::ShortDescriptor::new("game", "category"),
    }
}

fn splits() -> [Split; 3] {
    [
        Split::new(0, "s1", "Split 1"),
        Split::new(1, "s2", "Split 2"),
        Split::new(2, "s3", "Split 3"),
    ]
}

/// Makes a dummy session with the given observer.
pub(crate) fn make_session<'o, O: Observer>(observer: &'o O) -> Session<'static, 'o, O> {
    let splits = splits().into_iter().collect();
    let run = Run {
        metadata: game_category(),
        attempt: attempt_info(),
        splits,
    };
    Session::new(run, observer)
}

// TODO(@MattWindsor91): possibly unify with the presenter version.
#[derive(Clone, Default)]
pub struct Obs(Rc<RefCell<Vec<observer::Event>>>);

impl Obs {
    /// Asserts that the observer found a message that is equal to `evt`.
    pub fn assert_received(&self, evt: observer::Event) {
        let mut queue = self.0.borrow_mut();
        assert!(!queue.is_empty(), "expected an observation");
        assert_eq!(evt, queue.remove(0));
    }

    /// Asserts that the observer is empty.
    pub fn assert_empty(&self) {
        // We could use is_empty here, but this would report a less useful error
        // if there is an observation.
        assert_eq!(
            None,
            self.0.borrow().first(),
            "expected no more observations"
        )
    }
}

impl Observer for Obs {
    fn observe(&self, evt: Event) {
        self.0.borrow_mut().push(evt);
    }
}

/// Tests that receiving a dump multiple times, with no intervening actions, does not change the state.
#[test]
fn presenter_dump_idempotent() {
    let (obs, mut pump) = presenter::observer();
    let mut session = make_session(&obs);

    let mut p = presenter::Presenter::new(&mut session);

    let init_state = p.state.clone();
    p.action_handler.handle(Action::Dump);

    pump.pump(&mut p);
    let old_state = p.state.clone();
    assert_ne!(
        old_state, init_state,
        "dump should have altered initial state"
    );

    p.action_handler.handle(Action::Dump);

    pump.pump(&mut p);
    assert_eq!(
        old_state, p.state,
        "second dump should not have altered state any further"
    );
}
